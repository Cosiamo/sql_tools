use crate::{
    Error, QueryBuilder,
    sql_implementations::SQLiteConnect,
    statements::{
        create::{CreateColumns, CreateDataTypes, ModifyCreateTable},
        insert::{InsertProps, sql_implementations::oracle::validation::get_col_indexes},
    },
};

pub(crate) fn does_sqlite_table_exist(
    insert_props: &InsertProps,
    conn_info: &SQLiteConnect,
) -> Result<bool, Error> {
    // Does table exist
    match conn_info.table_info(&insert_props.table.to_string()) {
        Ok(_val) => Ok(true),
        Err(_err) => Ok(false),
    }
}

pub(crate) fn create_sqlite_table(
    insert_props: &InsertProps,
    conn_info: &SQLiteConnect,
) -> Result<(), Error> {
    let col_type_indexes = get_col_indexes(&insert_props.grid)?;
    let columns = &insert_props
        .header
        .iter()
        .enumerate()
        .map(|(idx, cell)| {
            if col_type_indexes.is_date.contains(&idx) {
                CreateColumns {
                    name: cell.to_string(),
                    data_type: CreateDataTypes::DATE,
                }
            } else if col_type_indexes.is_int.contains(&idx) {
                CreateColumns {
                    name: cell.to_string(),
                    data_type: CreateDataTypes::NUMBER,
                }
            } else if col_type_indexes.is_float.contains(&idx) {
                CreateColumns {
                    name: cell.to_string(),
                    data_type: CreateDataTypes::FLOAT,
                }
            } else {
                let size = if let Some(val) = col_type_indexes.varchar_size.get(&idx) {
                    val
                } else {
                    &(1 as usize)
                };
                CreateColumns {
                    name: cell.to_string(),
                    data_type: CreateDataTypes::VARCHAR(*size),
                }
            }
        })
        .collect::<Vec<CreateColumns>>();
    conn_info
        .create()
        .table(&insert_props.table, columns.to_vec())
        .build()?;
    Ok(())
}
