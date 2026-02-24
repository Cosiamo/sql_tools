use execution::stmt_res;

use crate::{
    Error, SQLImplementation,
    data_types::{SQLDataTypes, ToSQLData},
    statements::select::{
        SelectProps,
        sql_implementations::{
            extract_column_name, multithread::multithread_execution,
            mutate_query::limit_offset_oracle,
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
            let col = extract_column_name(col);
            Box::new(col.to_sql_fmt())
        })
        .collect::<Vec<Box<SQLDataTypes>>>();
    let columns = &cols.join(", ");

    let mut query = format!("SELECT {} FROM {}", &columns, &table,);

    let mut count_sql = format!("SELECT COUNT(*) FROM {}", &table);

    query = shared_select_operations(&select_props, query)?;
    count_sql = shared_select_operations(&select_props, count_sql)?;

    query = limit_offset_oracle(&select_props, query);
    count_sql = limit_offset_oracle(&select_props, count_sql);

    let conn_info = select_props.connect.as_oracle()?.clone();
    let conn = conn_info.initialize_connection()?;

    let mut count: Option<usize> = None;
    let count_query = conn.query(&count_sql, &[])?;
    for res in count_query {
        let row = res?;
        // might change get_as type to Option<usize>
        count = row.get_as::<Option<usize>>()?;
    }

    multithread_execution(
        SQLImplementation::Oracle(conn_info),
        oracle_handle_execution,
        select_props,
        header,
        query,
        count,
    )
}

pub(crate) fn oracle_build_single_thread_select(
    select_props: SelectProps,
) -> Result<Vec<Vec<Box<SQLDataTypes>>>, Error> {
    let table = &select_props.table;
    let cols = select_props.oracle_column_name()?;

    let header = &cols
        .iter()
        .map(|col| {
            let col = extract_column_name(col);
            Box::new(col.to_sql_fmt())
        })
        .collect::<Vec<Box<SQLDataTypes>>>();
    let columns = &cols.join(", ");

    let mut query = format!("SELECT {} FROM {}", &columns, &table,);

    query = shared_select_operations(&select_props, query)?;
    query = limit_offset_oracle(&select_props, query);

    let conn_info = select_props.connect.as_oracle()?;
    let conn = conn_info.initialize_connection()?;

    let stmt = conn.statement(&query).build()?;
    let column_size = header.len();
    let mut res = stmt_res(stmt, column_size, false)?;
    if select_props.return_header {
        let header = header
            .iter()
            .map(|head| {
                let head = head.to_string();
                let head = extract_column_name(&head);
                Box::new(head.to_sql_fmt())
            })
            .collect::<Vec<Box<SQLDataTypes>>>();
        let header = vec![header.to_owned()];
        res.splice(..0, header.iter().cloned());
    }
    Ok(res)
}
