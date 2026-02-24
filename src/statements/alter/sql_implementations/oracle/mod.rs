use crate::{Error, sql_implementations::OracleConnect};

pub(crate) fn alter_oracle(connect: OracleConnect, query: String) -> Result<(), Error> {
    let conn = connect.initialize_connection()?;
    conn.execute(&query, &[])?;
    Ok(())
}
