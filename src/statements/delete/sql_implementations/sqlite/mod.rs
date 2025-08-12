use rusqlite::Connection;

use crate::{Error, SQLVariation, statements::delete::DeleteProps};

pub fn sqlite_delete(props: DeleteProps) -> Result<(), Error> {
    let conn_info = match &props.connect {
        SQLVariation::Oracle(_) => return Err(Error::SQLVariationError),
        SQLVariation::SQLite(connect) => connect,
    };
    let conn = Connection::open(&conn_info.path.clone())?;

    let query = match props.clause {
        Some(filters) => format!("DELETE FROM {}.{} WHERE {}", &props.table.id, &props.table.name, filters),
        None => format!("DELETE FROM {}.{}", &props.table.id, &props.table.name),
    };
    conn.execute(&query, [])?;
    Ok(())
}
