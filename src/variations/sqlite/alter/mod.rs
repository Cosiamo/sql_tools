use rusqlite::Connection;

use crate::{Error, variations::SQLiteConnect};

pub(crate) fn alter_sqlite(connect: SQLiteConnect, query: String) -> Result<(), Error> {
    let conn = Connection::open(&connect.path)?;
    conn.execute(&query, [])?;
    Ok(())
}
