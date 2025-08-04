use crate::{Error, variations::OracleConnect};

pub(crate) fn alter_oracle(connect: OracleConnect, query: String) -> Result<(), Error> {
    let conn: oracle::Connection = oracle::Connection::connect(
        connect.username.clone(),
        connect.password.clone(),
        connect.connection_string.clone(),
    )?;
    conn.execute(&query, &[])?;
    Ok(())
}
