use crate::{data_types::{self, ToSQLData}, query_conjunctions::WhereArg};

pub(crate) fn where_clause_value_format<T: ToSQLData>(items: Vec<T>) -> String {
    items
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

pub(crate) fn where_match (
    column: &str, 
    values: WhereArg, 
) -> String {
    match values {
        WhereArg::Values(items) => {
            let value = where_clause_value_format(items);
            format!("{column} IN ({value})")
        },
        WhereArg::Like(like) => {
            format!("{column} LIKE '{like}'")
        },
        WhereArg::Query(value) => {
            format!("{column} IN ({value})")
        },
        WhereArg::NULL => {
            format!("{column} IS NULL")
        },
    }
}

pub(crate) fn where_match_not (
    column: &str, 
    values: WhereArg, 
) -> String {
    match values {
        WhereArg::Values(items) => {
            let value = where_clause_value_format(items);
            format!("{column} NOt IN ({value})")
        },
        WhereArg::Like(like) => {
            format!("{column} NOT LIKE '{like}'")
        },
        WhereArg::Query(value) => {
            format!("{column} NOT IN ({value})")
        },
        WhereArg::NULL => {
            format!("{column} IS NOT NULL")
        },
    }
}

pub(crate) fn conjunction_match (
    column: &str, 
    values: WhereArg, 
    clause: &Option<String>,
    conjunction: &str,
) -> String {
    match values {
        WhereArg::Values(items) => {
            let value = where_clause_value_format(items);
            let stmt = format!("{} IN ({})", column, value);
            if let Some(existing) = clause {
                format!("{existing} {conjunction} {stmt}")
            } else {
                format!("{stmt}")
            }
        },
        WhereArg::Like(like) => {
            let stmt = format!("{column} LIKE '{like}'");
            if let Some(existing) = clause {
                format!("{existing} {conjunction} {stmt}")
            } else {
                format!("{stmt}")
            }
        },
        WhereArg::Query(query) => {
            let stmt = format!("{column} IN ({query})");
            if let Some(existing) = clause {
                format!("{existing} {conjunction} {stmt}")
            } else {
                format!("{stmt}")
            }
        },
        WhereArg::NULL => {
            let stmt = format!("{column} IS NULL");
            if let Some(existing) = clause {
                format!("{existing} {conjunction} {stmt}")
            } else {
                format!("{stmt}")
            }
        },
    }
}

pub(crate) fn conjunction_match_not (
    column: &str, 
    values: WhereArg, 
    clause: &Option<String>,
    conjunction: &str,
) -> String {
    match values {
        WhereArg::Values(items) => {
            let value = where_clause_value_format(items);
            let stmt = format!("{} NOT IN ({})", column, value);
            if let Some(existing) = clause {
                format!("{existing} {conjunction} {stmt}")
            } else {
                format!("{stmt}")
            }
        },
        WhereArg::Like(like) => {
            let stmt = format!("{column} NOT LIKE '{like}'");
            if let Some(existing) = clause {
                format!("{existing} {conjunction} {stmt}")
            } else {
                format!("{stmt}")
            }
        },
        WhereArg::Query(query) => {
            let stmt = format!("{column} NOT IN ({query})");
            if let Some(existing) = clause {
                format!("{existing} {conjunction} {stmt}")
            } else {
                format!("{stmt}")
            }
        },
        WhereArg::NULL => {
            let stmt = format!("{column} IS NOT NULL");
            if let Some(existing) = clause {
                format!("{existing} {conjunction} {stmt}")
            } else {
                format!("{stmt}")
            }
        },
    }
}
