use crate::{Error, sql_implementations::SQLiteConnect};

pub(crate) fn alter_sqlite(connect: SQLiteConnect, query: String) -> Result<(), Error> {
    let conn = connect.initialize_connection()?;
    conn.execute(&query, [])?;
    Ok(())
}
