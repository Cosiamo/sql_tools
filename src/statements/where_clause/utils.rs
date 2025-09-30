use crate::data_types::{self, ToSQLData};

pub(crate) fn where_clause_value_format<T: ToSQLData>(values: Vec<T>) -> String {
    values
        .iter()
        .map(|cell| match cell.fmt_data() {
            data_types::SQLDataTypes::Varchar(val) => format!("'{}'", val),
            data_types::SQLDataTypes::Number(val) => format!("{}", val),
            data_types::SQLDataTypes::Float(val) => format!("{}", val),
            data_types::SQLDataTypes::Date(val) => format!("'{}'", val),
            data_types::SQLDataTypes::NULL => format!("NULL"),
        })
        .collect::<Vec<String>>()
        .join(", ")
}

pub(crate) fn match_table_ids(id: &String, column: &str) -> String {
    if column.contains(".") {
        column.to_owned()
    } else {
        format!("{id}.{column}")
    }
}