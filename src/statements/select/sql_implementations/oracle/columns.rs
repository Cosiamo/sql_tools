use crate::{Error, SQLImplementation, statements::select::SelectProps};

pub(crate) fn get_column_names_oracle(select_props: &SelectProps) -> Result<Vec<String>, Error> {
    let conn_info = match &select_props.connect {
        SQLImplementation::Oracle(connect) => connect,
        SQLImplementation::SQLite(_) => return Err(Error::SQLVariationError),
    };
    let sql = format!(
        "SELECT column_name FROM all_tab_columns WHERE UPPER(table_name) = '{}'",
        select_props.table.name.to_ascii_uppercase()
    );
    let conn: oracle::Connection = oracle::Connection::connect(
        conn_info.username.clone(),
        conn_info.password.clone(),
        conn_info.connection_string.clone(),
    )?;

    let mut header: Vec<String> = Vec::new();
    let rows = conn.query(&sql, &[])?;
    for row_result in rows {
        let row = row_result?;
        for val in row.sql_values() {
            let res = val.get::<String>()?;
            let col_id = format!("{}.{res}", select_props.table.id);
            header.push(col_id)
        }
    }
    Ok(header)
}
