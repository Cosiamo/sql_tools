use rusqlite::Connection;

use crate::{Error, sql_implementations::SQLiteConnect};

pub(crate) fn alter_sqlite(connect: SQLiteConnect, query: String) -> Result<(), Error> {
    let conn = Connection::open(&connect.path)?;
    conn.execute(&query, [])?;
    Ok(())
}
