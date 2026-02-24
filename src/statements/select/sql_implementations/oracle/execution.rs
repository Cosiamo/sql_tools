use std::sync::Arc;

use oracle::Statement;

use crate::{
    Error,
    data_types::SQLDataTypes,
    statements::select::SelectProps,
};

pub(crate) fn oracle_handle_execution(
    select_props: Arc<SelectProps>,
    column_size: usize,
    sql: String,
) -> Result<Vec<Vec<Box<SQLDataTypes>>>, Error> {
    let conn_info = select_props.connect.as_oracle()?;
    let conn = conn_info.initialize_connection()?;
    let stmt = conn.statement(&sql).build()?;
    stmt_res(stmt, column_size, true)
}

pub(crate) fn stmt_res(
    mut stmt: Statement,
    column_size: usize,
    _is_parallel: bool,
) -> Result<Vec<Vec<Box<SQLDataTypes>>>, Error> {
    let query = stmt.query(&[])?;
    let mut outer_vec = Vec::new();
    for v in query {
        let p = v?;
        let mut inner_vec = Vec::new();
        for col_idx in 0..column_size {
            let a = p.get::<usize, SQLDataTypes>(col_idx)?;
            inner_vec.push(Box::new(a))
        }
        // if is_parallel {
        //     inner_vec.remove(0);
        // }
        outer_vec.push(inner_vec)
    }

    Ok(outer_vec)
}
