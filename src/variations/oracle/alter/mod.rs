use crate::{variations::OracleConnect, Error};

pub(crate) fn alter(connect: OracleConnect, query: String) -> Result<(), Error> {
    let conn: oracle::Connection = oracle::Connection::connect(connect.username.clone(), connect.password.clone(), connect.connection_string.clone())?;
    conn.execute(&query, &[])?;
    Ok(())
}