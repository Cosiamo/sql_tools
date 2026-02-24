use crate::{Error, statements::delete::DeleteProps};

pub fn sqlite_delete(props: DeleteProps) -> Result<(), Error> {
    let conn_info = props.connect.as_sqlite()?;
    let conn = conn_info.initialize_connection()?;

    let query = match props.clause {
        Some(filters) => format!("DELETE FROM {} WHERE {}", &props.table, filters),
        None => format!("DELETE FROM {}", &props.table),
    };
    conn.execute(&query, [])?;
    Ok(())
}
