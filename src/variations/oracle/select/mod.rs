use std::thread::{self, JoinHandle};

use utils::stmt_res;

use crate::{data_types::SQLDataTypes, Error, SQLVariation};

use super::{OrderBy, SelectProps};

pub mod utils;

pub(crate) fn get_column_names_oracle(select_props: &SelectProps) -> Result<Vec<String>, Error> {
    let conn_info = match &select_props.connect {
        SQLVariation::Oracle(connect) => connect,
        SQLVariation::SQLite(_) => return Err(Error::SQLVariationError),
    };
    let sql = format!("SELECT column_name FROM all_tab_columns WHERE UPPER(table_name) = '{}'", select_props.table.to_ascii_uppercase());
    let conn: oracle::Connection = oracle::Connection::connect(conn_info.username.clone(), conn_info.password.clone(), conn_info.connection_string.clone())?;

    let mut header: Vec<String> = Vec::new();
    let rows = conn.query(&sql, &[])?;
    for row_result in rows {
        let row = row_result?;
        for val in row.sql_values() {
            let res = val.get()?;
            header.push(res)
        }
    };
    Ok(header)
}

pub(crate) fn oracle_build_select(mut select_props: SelectProps) -> Result<Vec<Vec<SQLDataTypes>>, Error> {
    let conn_info = match select_props.connect {
        SQLVariation::Oracle(ref connect) => connect,
        SQLVariation::SQLite(_) => return Err(Error::SQLVariationError),
    };

    if select_props.columns == vec!["*".to_string()] {
        select_props.columns = get_column_names_oracle(&select_props)?;
    }

    let mut query: String;

    let count_sql: String;
    match select_props.clause {
        Some(filters) => {
            count_sql = format!("SELECT COUNT(*) FROM {} WHERE {}", &select_props.table, &filters);
            query = format!("SELECT row_number() over (order by rowid) as rn, {} FROM {} WHERE {}", &select_props.columns.join(", "), &select_props.table, filters);
        },
        None => {
            count_sql = format!("SELECT COUNT(*) FROM {}", &select_props.table);
            query = format!("SELECT row_number() over (order by rowid) as rn, {} FROM {}", &select_props.columns.join(", "), &select_props.table);
        },
    }
    query = if let Some(group_by) = select_props.group_by { format!("{} GROUP BY {}, rowid", query, group_by.join(", ")) } else { query };
    
    match select_props.order_by {
        (None, OrderBy::ASC) => return Err(Error::OrderByError),
        (None, OrderBy::DESC) => return Err(Error::OrderByError),
        (None, OrderBy::None) => query = query,
        (Some(column), OrderBy::ASC) => query = format!("{} ORDER BY {} ASC", query, column),
        (Some(column), OrderBy::DESC) => query = format!("{} ORDER BY {} DESC", query, column),
        (Some(_), OrderBy::None) => query = query,
    }

    let mut count: Option<usize> = None;
    let conn: oracle::Connection = oracle::Connection::connect(&conn_info.username, &conn_info.password, &conn_info.connection_string).unwrap(); 
    let count_query = conn.query(&count_sql, &[])?;
    for res in count_query {
        let row = res?;
        count = Some(row.get_as::<usize>()?);
    };

    let len: usize = if let Some(val) = count { val } else { return Err(Error::CountError) };
    let nthreads = num_cpus::get();
    let num = (len / nthreads + if len % nthreads == 0 { 0 } else { 1 }) as f32;

    let mut handles: Vec<JoinHandle<Result<Vec<Vec<SQLDataTypes>>, Error>>> = Vec::new();

    let mut c: usize = 0;
    let mut prev: usize = 0;

    let col_len = select_props.columns.len() + 1;

    for n in 0..nthreads {
        let start: usize;
        if n == 0 { start = 1 }
        else { start = prev + 1 }
        let mut end = (c + 1) * num.ceil() as usize;
        if end > len { end = len }
        // println!("Start:{}  End:{}", start, end);
        let stmt = format!("SELECT * FROM ({}) WHERE rn >= {} and rn <= {}", query, start, end);
        // println!("{:?}", stmt);
        let username = conn_info.username.to_owned();
        let password = conn_info.password.to_owned();
        let connection_string = conn_info.connection_string.to_owned();
        
        handles.push(thread::spawn(move || {
            let conn: oracle::Connection = oracle::Connection::connect(username, password, connection_string).unwrap(); 
            let stmt = conn.statement(&stmt).build()?;
            stmt_res(stmt, col_len)
        }));
        prev = end;
        c += 1;
    }

    let mut group = Vec::new();
    for handle in handles {
        let mut handle = handle.join().unwrap()?;
        let res = handle.iter_mut().map(|row|{
            let _ = row.remove(0);
            row.to_owned()
        }).collect::<Vec<Vec<SQLDataTypes>>>();
        group.push(res);
    }
    let res = group.concat();
    // res.iter().for_each(|c|{ println!("{:?}", c) });

    Ok(res)
}

pub(crate) fn oracle_build_single_thread_select(select_props: SelectProps) -> Result<Vec<Vec<SQLDataTypes>>, Error> {
    let conn_info = match select_props.connect {
        SQLVariation::Oracle(oracle_connect) => oracle_connect,
        SQLVariation::SQLite(_) => return Err(Error::SQLVariationError),
    };
    let mut query = match select_props.clause {
        Some(filters) => format!("SELECT {} FROM {} WHERE {}", &select_props.columns.join(", "), &select_props.table, filters),
        None => format!("SELECT {} FROM {}", &select_props.columns.join(", "), &select_props.table),
    };

    query = if let Some(group_by) = select_props.group_by { format!("{} GROUP BY {}", query, group_by.join(", ")) } else { query };

    match select_props.order_by {
        (None, OrderBy::ASC) => return Err(Error::OrderByError),
        (None, OrderBy::DESC) => return Err(Error::OrderByError),
        (None, OrderBy::None) => query = query,
        (Some(column), OrderBy::ASC) => query = format!("{} ORDER BY {} ASC", query, column),
        (Some(column), OrderBy::DESC) => query = format!("{} ORDER BY {} DESC", query, column),
        (Some(_), OrderBy::None) => query = query,
    }

    let conn: oracle::Connection = oracle::Connection::connect(conn_info.username, conn_info.password, conn_info.connection_string).unwrap(); 
    let stmt = conn.statement(&query).build()?;
    stmt_res(stmt, select_props.columns.len())
}