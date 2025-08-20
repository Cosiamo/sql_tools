use crate::{Error, SQLImplementation, statements::delete::DeleteProps};

pub fn oracle_build_delete(props: DeleteProps) -> Result<(), Error> {
    let conn_info = match props.connect {
        SQLImplementation::Oracle(oracle_connect) => oracle_connect,
        SQLImplementation::SQLite(_) => return Err(Error::SQLVariationError),
    };
    let conn: oracle::Connection = oracle::Connection::connect(
        &conn_info.username,
        &conn_info.password,
        &conn_info.connection_string,
    )
    .unwrap();

    let sql = match props.clause {
        Some(filters) => format!("DELETE FROM {}.{} WHERE {}", &props.table.id, &props.table.name, filters),
        None => format!("DELETE FROM {}.{}", &props.table.id, &props.table.name),
    };
    conn.execute(&sql, &[])?;
    conn.commit()?;
    Ok(())
}
