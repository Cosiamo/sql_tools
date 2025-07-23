use std::thread::{self, JoinHandle};

use utils::stmt_res;

use crate::{data_types::SQLDataTypes, variations::{oracle::select::{columns::get_column_names_oracle, mutate_query::{filters, group_by, join_operations, limit_offset, order_by}}, OracleConnect}, Error, SQLVariation};

use super::SelectProps;

pub mod utils;
pub mod columns;
pub mod mutate_query;

pub(crate) fn oracle_build_select(
    mut select_props: SelectProps,
) -> Result<Vec<Vec<Box<SQLDataTypes>>>, Error> {
    // ===== Get all column names =====
    if select_props.columns == vec!["*".to_string()] {
        select_props.columns = get_column_names_oracle(&select_props)?;
    }

    // ===== Initialize Queries =====
    let mut query = format!(
        "SELECT row_number() over (order by rowid) as rn, {} FROM {}",
        &select_props.columns.join(", "),
        &select_props.table.query_fmt()
    );
    let mut count_sql = format!("SELECT COUNT(*) FROM {}", &select_props.table.query_fmt());

    // ===== Joins =====
    if &select_props.joins.len() > &0 {
        query = join_operations(&select_props, query);
    }
    if &select_props.joins.len() > &0 {
        count_sql = join_operations(&select_props, count_sql);
    }

    // ===== If filters =====
    query = filters(&select_props, &query);
    count_sql = filters(&select_props, &count_sql);

    // ===== Group By =====
    query = group_by(&select_props, &query);
    count_sql = group_by(&select_props, &count_sql);

    // ===== Order By =====
    query = order_by(&select_props, &query)?;
    count_sql = order_by(&select_props, &count_sql)?;

    // ===== Limit Offset =====
    query = limit_offset(&select_props, query);
    count_sql = limit_offset(&select_props, count_sql);

    // ===== Initialize connection =====
    let conn_info = extract_connection(&select_props.connect)?;
    let conn: oracle::Connection = oracle::Connection::connect(
        &conn_info.username,
        &conn_info.password,
        &conn_info.connection_string,
    )?;

    // ===== Get number of rows =====
    let mut count: Option<usize> = None;
    let count_query = conn.query(&count_sql, &[])?;
    for res in count_query {
        let row = res?;
        // might change get_as type to Option<usize>
        count = row.get_as::<Option<usize>>()?;
    }

    // ===== Multi-threading functionality =====
    let len: usize = if let Some(val) = count {
        val
    } else {
        return Err(Error::CountError);
    };
    let nthreads = num_cpus::get();
    let num = (len / nthreads + if len % nthreads == 0 { 0 } else { 1 }) as f32;

    let mut handles: Vec<JoinHandle<Result<Vec<Vec<Box<SQLDataTypes>>>, Error>>> = Vec::new();

    let mut c: usize = 0;
    let mut prev: usize = 0;

    let col_len = select_props.columns.len() + 1;

    for n in 0..nthreads {
        let start: usize;
        if n == 0 {
            start = 1
        } else {
            start = prev + 1
        }
        let mut end = (c + 1) * num.ceil() as usize;
        if end > len {
            end = len
        }
        // println!("Start:{}  End:{}", start, end);
        let stmt = format!(
            "SELECT * FROM ({}) WHERE rn >= {} and rn <= {}",
            query, start, end
        );
        // println!("{:?}", stmt);
        let username = conn_info.username.to_owned();
        let password = conn_info.password.to_owned();
        let connect_string = conn_info.connection_string.to_owned();

        handles.push(thread::spawn(move || {
            let conn: oracle::Connection =
                oracle::Connection::connect(username, password, connect_string)?;
            let stmt = conn.statement(&stmt).build()?;
            stmt_res(stmt, col_len)
        }));
        prev = end;
        c += 1;
    }

    let mut group = Vec::new();
    for handle in handles {
        let mut handle = handle.join().unwrap()?;
        let res = handle
            .iter_mut()
            .map(|row| {
                let _ = row.remove(0);
                row.to_owned()
            })
            .collect::<Vec<Vec<Box<SQLDataTypes>>>>();
        group.push(res);
    }
    let res = group.concat();
    // res.iter().for_each(|c|{ println!("{:?}", c) });

    Ok(res)
}

pub(crate) fn oracle_build_single_thread_select(
    mut select_props: SelectProps,
) -> Result<Vec<Vec<Box<SQLDataTypes>>>, Error> {
    // ===== Get all column names =====
    if select_props.columns == vec!["*".to_string()] {
        select_props.columns = get_column_names_oracle(&select_props)?;
    }

    // ===== Initialize query =====
    let mut query = format!(
        "SELECT {} FROM {}",
        &select_props.columns.join(", "),
        &select_props.table.query_fmt()
    );

    // ===== Joins =====
    if &select_props.joins.len() > &0 {
        query = join_operations(&select_props, query);
    }

    // ===== If filters =====
    query = filters(&select_props, &query);

    // ===== Group By =====
    query = group_by(&select_props, &query);

    // ===== Order By =====
    query = order_by(&select_props, &query)?;

    // ===== Limit Offset =====
    query = limit_offset(&select_props, query);

    // ===== Initialize connection =====
    let conn_info = extract_connection(&select_props.connect)?;
    let conn: oracle::Connection = oracle::Connection::connect(
        conn_info.username,
        conn_info.password,
        conn_info.connection_string,
    )?;
    
    // ===== Run query =====
    let stmt = conn.statement(&query).build()?;
    stmt_res(stmt, select_props.columns.len())
}

fn extract_connection(connect: &SQLVariation) -> Result<OracleConnect, Error> {
    match connect {
        SQLVariation::Oracle(oracle_connect) => Ok(oracle_connect.to_owned()),
        SQLVariation::SQLite(_) => return Err(Error::SQLVariationError),
    }
}