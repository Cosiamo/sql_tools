use execution::stmt_res;

use crate::{
    Error, SQLImplementation,
    data_types::{SQLDataTypes, ToSQLData},
    sql_implementations::OracleConnect,
    statements::select::{
        SelectProps,
        sql_implementations::{
            multithread::multithread_execution, mutate_query::limit_offset_oracle,
            oracle::execution::oracle_handle_execution, shared_select_operations,
        },
    },
};

pub mod columns;
pub mod execution;

pub(crate) fn oracle_build_select(
    select_props: SelectProps,
) -> Result<Vec<Vec<Box<SQLDataTypes>>>, Error> {
    let table = &select_props.table;
    let cols = select_props.oracle_column_name()?;

    let header = &cols
        .iter()
        .map(|col| {
            let col = col.split(".").collect::<Vec<&str>>();
            let col = col[col.len() - 1];
            let col = col.split(" ").collect::<Vec<&str>>();
            let col = col[col.len() - 1];
            let col = col.split(" as ").collect::<Vec<&str>>();
            let col = col[col.len() - 1];
            Box::new(col.to_sql_fmt())
        })
        .collect::<Vec<Box<SQLDataTypes>>>();
    let columns = &cols.join(", ");
    
    let mut query = format!("SELECT {} FROM {}", &columns, &table,);
    dbg!(&query);

    dbg!(&columns);

    let mut count_sql = format!("SELECT COUNT(*) FROM {}", &table);

    query = shared_select_operations(&select_props, query)?;
    count_sql = shared_select_operations(&select_props, count_sql)?;

    query = limit_offset_oracle(&select_props, query);
    count_sql = limit_offset_oracle(&select_props, count_sql);

    let conn_info = extract_connection(&select_props.connect)?;
    let conn: oracle::Connection = oracle::Connection::connect(
        &conn_info.username,
        &conn_info.password,
        &conn_info.connection_string,
    )?;

    let mut count: Option<usize> = None;
    let count_query = conn.query(&count_sql, &[])?;
    for res in count_query {
        let row = res?;
        // might change get_as type to Option<usize>
        count = row.get_as::<Option<usize>>()?;
    }

    multithread_execution( SQLImplementation::Oracle(conn_info),oracle_handle_execution, select_props, header, query, count)
}

pub(crate) fn oracle_build_single_thread_select(
    select_props: SelectProps,
) -> Result<Vec<Vec<Box<SQLDataTypes>>>, Error> {
    let table = &select_props.table;
    let cols = select_props.oracle_column_name()?;

    let header = &cols
        .iter()
        .map(|col| {
            let col = col.split(".").collect::<Vec<&str>>();
            let col = col[col.len() - 1];
            let col = col.split(" ").collect::<Vec<&str>>();
            let col = col[col.len() - 1];
            let col = col.split(" as ").collect::<Vec<&str>>();
            let col = col[col.len() - 1];
            Box::new(col.to_sql_fmt())
        })
        .collect::<Vec<Box<SQLDataTypes>>>();
    let columns = &cols.join(", ");

    let mut query = format!("SELECT {} FROM {}", &columns, &table,);

    query = shared_select_operations(&select_props, query)?;
    query = limit_offset_oracle(&select_props, query);
    dbg!(&query);

    let conn_info = extract_connection(&select_props.connect)?;
    let conn: oracle::Connection = oracle::Connection::connect(
        conn_info.username,
        conn_info.password,
        conn_info.connection_string,
    )?;

    let stmt = conn.statement(&query).build()?;
    let column_size = header.len();
    let mut res = stmt_res(stmt, column_size, false)?;
    if select_props.return_header {
        let header = header
            .iter()
            .map(|head| {
                let head = head.to_string();
                let head = head.split(" as ").collect::<Vec<&str>>();
                let head = head[head.len() - 1];
                let head = head.split(" ").collect::<Vec<&str>>();
                let head = head[head.len() - 1];
                let head = head.split(".").collect::<Vec<&str>>();
                let head = head[head.len() - 1];
                Box::new(head.to_sql_fmt())
            })
            .collect::<Vec<Box<SQLDataTypes>>>();
        let header = vec![header.to_owned()];
        res.splice(..0, header.iter().cloned());
    }
    Ok(res)
}

fn extract_connection(connect: &SQLImplementation) -> Result<OracleConnect, Error> {
    match connect {
        SQLImplementation::Oracle(oracle_connect) => Ok(oracle_connect.to_owned()),
        SQLImplementation::SQLite(_) => return Err(Error::SQLVariationError),
    }
}
