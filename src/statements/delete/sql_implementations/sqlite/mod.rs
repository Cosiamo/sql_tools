use crate::{Error, SQLImplementation, statements::delete::DeleteProps};

pub fn sqlite_delete(props: DeleteProps) -> Result<(), Error> {
    let conn_info = match &props.connect {
        SQLImplementation::Oracle(_) => return Err(Error::SQLVariationError),
        SQLImplementation::SQLite(connect) => connect,
    };
    let conn = conn_info.initialize_connection()?;

    let query = match props.clause {
        Some(filters) => format!("DELETE FROM {} WHERE {}", &props.table, filters),
        None => format!("DELETE FROM {}", &props.table),
    };
    conn.execute(&query, [])?;
    Ok(())
}
