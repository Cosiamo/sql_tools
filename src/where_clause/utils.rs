use crate::data_types::{self, ToSQLData};

pub fn where_clause_value_format<T: ToSQLData>(values: Vec<T>) -> String {
    values.iter().map(|cell| {
        match cell.fmt_data_borrowed() {
            data_types::SQLDataTypes::VARCHAR(val) => format!("'{}'", val),
            data_types::SQLDataTypes::NUMBER(val) => format!("{}", val),
            data_types::SQLDataTypes::FLOAT(val) => format!("{}", val),
            data_types::SQLDataTypes::DATE(val) => format!("'{}'", val),
            data_types::SQLDataTypes::NULL => format!("NULL"),
        }
    }).collect::<Vec<String>>().join(", ")
}