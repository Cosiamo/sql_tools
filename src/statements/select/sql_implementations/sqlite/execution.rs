use std::sync::Arc;

use crate::{Error, SQLImplementation, data_types::SQLDataTypes, statements::select::SelectProps};

pub fn sqlite_handle_execution(
    select_props: Arc<SelectProps>,
    column_size: usize,
    sql: String,
) -> Result<Vec<Vec<Box<SQLDataTypes>>>, Error> {
    let conn_info = match &select_props.connect {
        SQLImplementation::Oracle(_) => return Err(Error::SQLVariationError),
        SQLImplementation::SQLite(connect) => connect,
    };
    let conn = conn_info.initialize_connection()?;
    let mut stmt = conn.prepare(&sql)?;
    let mut rows = stmt.query([])?;
    let mut res = Vec::new();
    while let Some(row) = rows.next()? {
        let mut p = Vec::new();
        for idx in 0..column_size {
            p.push(Box::new(row.get::<usize, SQLDataTypes>(idx).unwrap()))
        }
        p.remove(0); // Removes the row number
        res.push(p)
    }
    Ok(res)
}
