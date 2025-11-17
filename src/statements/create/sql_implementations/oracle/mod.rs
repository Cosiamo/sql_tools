use crate::{
    Error, SQLImplementation,
    statements::create::{CreateTable, sql_implementations::utils::fmt_create_table_columns},
};

pub(crate) fn oracle_build_create_table(create_table: CreateTable) -> Result<(), Error> {
    let conn_info = match &create_table.connect {
        SQLImplementation::Oracle(oracle_connect) => oracle_connect,
        SQLImplementation::SQLite(_) => return Err(Error::SQLVariationError),
    };

    let sql = fmt_create_table_columns(&create_table);
    let conn: oracle::Connection = oracle::Connection::connect(
        &conn_info.username,
        &conn_info.password,
        &conn_info.connection_string,
    )
    .unwrap();
    conn.execute(&sql, &[])?;
    conn.commit()?;
    Ok(())
}
