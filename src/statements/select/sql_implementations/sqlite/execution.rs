use std::sync::Arc;

use rusqlite::Connection;

use crate::{Error, SQLVariation, data_types::SQLDataTypes, statements::select::SelectProps};

pub fn sqlite_handle_execution(
    select_props: Arc<SelectProps>,
    sql: String,
) -> Result<Vec<Vec<Box<SQLDataTypes>>>, Error> {
    let path = match &select_props.connect {
        SQLVariation::Oracle(_) => return Err(Error::SQLVariationError),
        SQLVariation::SQLite(connect) => &connect.path,
    };
    let conn = Connection::open(path.clone())?;
    let column_size = select_props.columns.len() + 1;
    let mut stmt = conn.prepare(&sql)?;
    let mut rows = stmt.query([])?;
    let mut res = Vec::new();
    while let Some(row) = rows.next()? {
        // let p = select_props.columns.iter().enumerate().map(|(idx, _)| {
        //     row.get::<usize, SQLDataTypes>(idx).unwrap()
        // }).collect::<Vec<SQLDataTypes>>();
        let mut p = Vec::new();
        for idx in 0..column_size {
            p.push(Box::new(row.get::<usize, SQLDataTypes>(idx).unwrap()))
        }
        res.push(p)
    }
    Ok(res)
}
