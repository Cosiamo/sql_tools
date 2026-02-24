use crate::{
    Error,
    statements::create::{CreateTable, sql_implementations::utils::fmt_create_table_columns},
};

pub(crate) fn oracle_build_create_table(create_table: CreateTable) -> Result<(), Error> {
    let conn_info = create_table.connect.as_oracle()?;

    let sql = fmt_create_table_columns(&create_table);
    let conn = conn_info.initialize_connection()?;
    conn.execute(&sql, &[])?;
    conn.commit()?;
    Ok(())
}
