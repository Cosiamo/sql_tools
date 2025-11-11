use crate::statements::create::{CreateDataTypes, CreateTable};

pub(crate) fn fmt_create_table_columns(create_table: &CreateTable) -> String {
    let cols_and_data_types = create_table
        .columns
        .iter()
        .map(|col_props| match col_props.data_type {
            CreateDataTypes::VARCHAR(mut num) => {
                if num == 0 {
                    num = 1
                } 
                format!("{} VARCHAR2({})", &col_props.name, num)
            }
            CreateDataTypes::NUMBER => format!("{} NUMBER", &col_props.name),
            CreateDataTypes::FLOAT => format!("{} FLOAT", &col_props.name),
            CreateDataTypes::DATE => format!("{} DATE", &col_props.name),
        })
        .collect::<Vec<String>>()
        .join(", ");

    format!(
        "CREATE TABLE {} ({})",
        create_table.table, cols_and_data_types
    )
}