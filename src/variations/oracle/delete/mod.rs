use crate::{Error, SQLVariation, clauses::delete::DeleteProps};

pub fn oracle_build_delete(props: DeleteProps) -> Result<(), Error> {
    let conn_info = match props.connect {
        SQLVariation::Oracle(oracle_connect) => oracle_connect,
        SQLVariation::SQLite(_) => return Err(Error::SQLVariationError),
    };
    let conn: oracle::Connection = oracle::Connection::connect(
        &conn_info.username,
        &conn_info.password,
        &conn_info.connection_string,
    )
    .unwrap();

    let sql = match props.clause {
        Some(filters) => format!("DELETE FROM {} WHERE {}", &props.table, filters),
        None => format!("DELETE FROM {}", &props.table),
    };
    conn.execute(&sql, &[])?;
    conn.commit()?;
    Ok(())
}
