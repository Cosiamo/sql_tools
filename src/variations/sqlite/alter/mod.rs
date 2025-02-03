use rusqlite::Connection;

use crate::{variations::SQLiteConnect, Error};

pub(crate) fn alter_sqlite(connect: SQLiteConnect, query: String) -> Result<(), Error> {
    let conn = Connection::open(&connect.path)?;
    conn.execute(&query, [])?;
    Ok(())
}