use execution::stmt_res;

use crate::{
    Error, SQLImplementation,
    data_types::SQLDataTypes,
    sql_implementations::OracleConnect,
    statements::select::{
        SelectProps,
        sql_implementations::{
            multithread::multithread_execution,
            mutate_query::limit_offset_oracle,
            oracle::{columns::get_column_names_oracle, execution::oracle_handle_execution},
            shared_select_operations,
        },
    },
};

pub mod columns;
pub mod execution;

pub(crate) fn oracle_build_select(
    mut select_props: SelectProps,
) -> Result<Vec<Vec<Box<SQLDataTypes>>>, Error> {
    if select_props.columns[0].name == "*".to_string() {
        select_props.columns = get_column_names_oracle(&select_props)?;
    }

    let table = &select_props.table;
    let cols = &select_props
        .columns
        .iter()
        .map(|col| format!("{}.{}", col.table, col.name))
        .collect::<Vec<String>>();

    let mut query = format!(
        "SELECT row_number() over (order by {}.rowid) as row_num, {} FROM {}",
        &table,
        &cols.join(", "),
        &table,
    );

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

    multithread_execution(oracle_handle_execution, select_props, query, count)
}

pub(crate) fn oracle_build_single_thread_select(
    mut select_props: SelectProps,
) -> Result<Vec<Vec<Box<SQLDataTypes>>>, Error> {
    if select_props.columns[0].name == "*".to_string() {
        select_props.columns = get_column_names_oracle(&select_props)?;
    }

    let table = &select_props.table;
    let cols = &select_props
        .columns
        .iter()
        .map(|col| format!("{}.{}", col.table, col.name))
        .collect::<Vec<String>>();

    let mut query = format!("SELECT {} FROM {}", &cols.join(", "), &table,);

    query = shared_select_operations(&select_props, query)?;

    query = limit_offset_oracle(&select_props, query);

    let conn_info = extract_connection(&select_props.connect)?;
    let conn: oracle::Connection = oracle::Connection::connect(
        conn_info.username,
        conn_info.password,
        conn_info.connection_string,
    )?;

    let stmt = conn.statement(&query).build()?;
    let mut res = stmt_res(stmt, select_props.columns.len(), false)?;
    if select_props.return_header {
        let header = vec![
            select_props
                .columns
                .iter()
                .map(|column| Box::new(SQLDataTypes::Varchar(column.name.to_string())))
                .collect::<Vec<Box<SQLDataTypes>>>(),
        ];
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
