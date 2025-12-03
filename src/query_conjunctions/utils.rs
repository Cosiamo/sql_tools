use crate::{
    data_types::{self, ToSQLData},
    query_conjunctions::WhereArg,
    statements::select::ColumnProps,
};

pub(crate) fn where_clause_value_format<T: ToSQLData>(items: Vec<T>) -> String {
    items
        .iter()
        .map(|cell| match cell.to_sql_fmt() {
            data_types::SQLDataTypes::Varchar(val) => format!("'{}'", val),
            data_types::SQLDataTypes::Number(val) => format!("{}", val),
            data_types::SQLDataTypes::Float(val) => format!("{}", val),
            data_types::SQLDataTypes::Date(val) => format!("'{}'", val),
            data_types::SQLDataTypes::NULL => format!("NULL"),
        })
        .collect::<Vec<String>>()
        .join(", ")
}

pub(crate) fn where_match(column: &ColumnProps, values: WhereArg) -> String {
    let col: String;
    if column.table.len() == 0 {
        col = format!("{}", column.name);
    } else {
        col = format!("{}.{}", column.table, column.name);
    }

    match values {
        WhereArg::Values(items) => {
            let value = where_clause_value_format(items);
            format!("{col} IN ({value})")
        }
        WhereArg::Like(like) => {
            format!("{col} LIKE '{like}'")
        }
        WhereArg::Query(value) => {
            format!("{col} IN ({value})")
        }
        WhereArg::NULL => {
            format!("{col} IS NULL")
        }
    }
}

pub(crate) fn where_match_not(column: &ColumnProps, values: WhereArg) -> String {
    let col: String;
    if column.table.len() == 0 {
        col = format!("{}", column.name);
    } else {
        col = format!("{}.{}", column.table, column.name);
    }

    match values {
        WhereArg::Values(items) => {
            let value = where_clause_value_format(items);
            format!("{col} NOT IN ({value})")
        }
        WhereArg::Like(like) => {
            format!("{col} NOT LIKE '{like}'")
        }
        WhereArg::Query(value) => {
            format!("{col} NOT IN ({value})")
        }
        WhereArg::NULL => {
            format!("{col} IS NOT NULL")
        }
    }
}

pub(crate) fn conjunction_match(
    column: &ColumnProps,
    values: WhereArg,
    clause: &Option<String>,
    conjunction: &str,
) -> String {
    let col: String;
    if column.table.len() == 0 {
        col = format!("{}", column.name);
    } else {
        col = format!("{}.{}", column.table, column.name);
    }

    match values {
        WhereArg::Values(items) => {
            let value = where_clause_value_format(items);
            let stmt = format!("{} IN ({})", col, value);
            if let Some(existing) = clause {
                format!("{existing} {conjunction} {stmt}")
            } else {
                format!("{stmt}")
            }
        }
        WhereArg::Like(like) => {
            let stmt = format!("{col} LIKE '{like}'");
            if let Some(existing) = clause {
                format!("{existing} {conjunction} {stmt}")
            } else {
                format!("{stmt}")
            }
        }
        WhereArg::Query(query) => {
            let stmt = format!("{col} IN ({query})");
            if let Some(existing) = clause {
                format!("{existing} {conjunction} {stmt}")
            } else {
                format!("{stmt}")
            }
        }
        WhereArg::NULL => {
            let stmt = format!("{col} IS NULL");
            if let Some(existing) = clause {
                format!("{existing} {conjunction} {stmt}")
            } else {
                format!("{stmt}")
            }
        }
    }
}

pub(crate) fn conjunction_match_not(
    column: &ColumnProps,
    values: WhereArg,
    clause: &Option<String>,
    conjunction: &str,
) -> String {
    let col: String;
    if column.table.len() == 0 {
        col = format!("{}", column.name);
    } else {
        col = format!("{}.{}", column.table, column.name);
    }

    match values {
        WhereArg::Values(items) => {
            let value = where_clause_value_format(items);
            let stmt = format!("{} NOT IN ({})", col, value);
            if let Some(existing) = clause {
                format!("{existing} {conjunction} {stmt}")
            } else {
                format!("{stmt}")
            }
        }
        WhereArg::Like(like) => {
            let stmt = format!("{col} NOT LIKE '{like}'");
            if let Some(existing) = clause {
                format!("{existing} {conjunction} {stmt}")
            } else {
                format!("{stmt}")
            }
        }
        WhereArg::Query(query) => {
            let stmt = format!("{col} NOT IN ({query})");
            if let Some(existing) = clause {
                format!("{existing} {conjunction} {stmt}")
            } else {
                format!("{stmt}")
            }
        }
        WhereArg::NULL => {
            let stmt = format!("{col} IS NOT NULL");
            if let Some(existing) = clause {
                format!("{existing} {conjunction} {stmt}")
            } else {
                format!("{stmt}")
            }
        }
    }
}
