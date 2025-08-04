use execution::stmt_res;

use crate::{data_types::SQLDataTypes, statements::select::{sql_implementations::{multithread_execution, mutate_query::limit_offset_oracle, oracle::{columns::get_column_names_oracle, execution::oracle_handle_execution}, shared_select_operations}, SelectProps}, sql_variations::OracleConnect, Error, SQLVariation};

pub mod execution;
pub mod columns;

pub(crate) fn oracle_build_select(
    mut select_props: SelectProps,
) -> Result<Vec<Vec<Box<SQLDataTypes>>>, Error> {
    // ===== Get all column names =====
    if select_props.columns == vec!["*".to_string()] {
        select_props.columns = get_column_names_oracle(&select_props)?;
    }

    // ===== Initialize Queries =====
    let mut query = format!(
        "SELECT row_number() over (order by {}.rowid) as row_num, {} FROM {}",
        &select_props.table.id,
        &select_props.columns.join(", "),
        &select_props.table.query_fmt()
    );

    let mut count_sql = format!("SELECT COUNT(*) FROM {}", &select_props.table.query_fmt());

    // ===== Select Operations =====
    query = shared_select_operations(&select_props, query)?;
    count_sql = shared_select_operations(&select_props, count_sql)?;

    // ===== Limit Offset =====
    query = limit_offset_oracle(&select_props, query);
    count_sql = limit_offset_oracle(&select_props, count_sql);

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
    multithread_execution(oracle_handle_execution, select_props, query, count)
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

    // ===== Select Operations =====
    query = shared_select_operations(&select_props, query)?;

    // ===== Limit Offset =====
    query = limit_offset_oracle(&select_props, query);

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