use rusqlite::Connection;

use crate::{
    Error, SQLImplementation,
    statements::create::{CreateTable, sql_implementations::utils::fmt_create_table_columns},
};

pub(crate) fn sqlite_build_create_table(create_table: CreateTable) -> Result<(), Error> {
    let conn_info = match &create_table.connect {
        SQLImplementation::Oracle(_) => return Err(Error::SQLVariationError),
        SQLImplementation::SQLite(connect) => connect,
    };

    let sql = fmt_create_table_columns(&create_table);
    let conn = Connection::open(&conn_info.path)?;
    conn.execute(&sql, ())?;
    Ok(())
}
