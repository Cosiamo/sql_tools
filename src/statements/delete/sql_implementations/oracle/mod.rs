use crate::{Error, statements::delete::DeleteProps};

pub fn oracle_build_delete(props: DeleteProps) -> Result<(), Error> {
    let conn_info = props.connect.as_oracle()?;
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
