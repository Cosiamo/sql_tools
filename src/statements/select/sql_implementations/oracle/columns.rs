use crate::{
    Error,
    statements::select::{ColumnProps, SelectProps},
};

pub fn get_column_names_oracle(select_props: &SelectProps) -> Result<Vec<ColumnProps>, Error> {
    let conn_info = select_props.connect.as_oracle()?;
    let sql = format!(
        "SELECT column_name FROM all_tab_columns WHERE UPPER(table_name) = '{}'",
        select_props.table.to_ascii_uppercase()
    );
    let conn: oracle::Connection = oracle::Connection::connect(
        conn_info.username.clone(),
        conn_info.password.clone(),
        conn_info.connection_string.clone(),
    )?;

    let mut header: Vec<ColumnProps> = Vec::new();
    let rows = conn.query(&sql, &[])?;
    for row_result in rows {
        let row = row_result?;
        for val in row.sql_values() {
            let res = val.get::<String>()?;
            header.push(ColumnProps {
                name: res,
                table: select_props.table.to_owned(),
            })
        }
    }
    Ok(header)
}
