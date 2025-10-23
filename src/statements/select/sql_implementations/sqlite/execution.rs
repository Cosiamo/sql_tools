use std::sync::Arc;

use rusqlite::Connection;

use crate::{Error, SQLImplementation, data_types::SQLDataTypes, statements::select::SelectProps};

pub fn sqlite_handle_execution(
    select_props: Arc<SelectProps>,
    sql: String,
) -> Result<Vec<Vec<Box<SQLDataTypes>>>, Error> {
    let path = match &select_props.connect {
        SQLImplementation::Oracle(_) => return Err(Error::SQLVariationError),
        SQLImplementation::SQLite(connect) => &connect.path,
    };
    let conn = Connection::open(path.clone())?;
    let column_size = select_props.columns.len() + 1;
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
