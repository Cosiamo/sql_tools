use crate::{
    data_types::{self, ToSQLData},
    query_conjunctions::WhereArg,
    statements::select::ColumnProps,
};

fn format_column(column: &ColumnProps) -> String {
    if column.table.is_empty() {
        column.name.clone()
    } else {
        format!("{}.{}", column.table, column.name)
    }
}

fn append_clause(clause: &Option<String>, conjunction: &str, stmt: &str) -> String {
    if let Some(existing) = clause {
        format!("{existing} {conjunction} {stmt}")
    } else {
        stmt.to_string()
    }
}

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
    let col = format_column(column);

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
    let col = format_column(column);

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
    let col = format_column(column);

    match values {
        WhereArg::Values(items) => {
            let value = where_clause_value_format(items);
            let stmt = format!("{} IN ({})", col, value);
            append_clause(clause, conjunction, &stmt)
        }
        WhereArg::Like(like) => {
            let stmt = format!("{col} LIKE '{like}'");
            append_clause(clause, conjunction, &stmt)
        }
        WhereArg::Query(query) => {
            let stmt = format!("{col} IN ({query})");
            append_clause(clause, conjunction, &stmt)
        }
        WhereArg::NULL => {
            let stmt = format!("{col} IS NULL");
            append_clause(clause, conjunction, &stmt)
        }
    }
}

pub(crate) fn conjunction_match_not(
    column: &ColumnProps,
    values: WhereArg,
    clause: &Option<String>,
    conjunction: &str,
) -> String {
    let col = format_column(column);

    match values {
        WhereArg::Values(items) => {
            let value = where_clause_value_format(items);
            let stmt = format!("{} NOT IN ({})", col, value);
            append_clause(clause, conjunction, &stmt)
        }
        WhereArg::Like(like) => {
            let stmt = format!("{col} NOT LIKE '{like}'");
            append_clause(clause, conjunction, &stmt)
        }
        WhereArg::Query(query) => {
            let stmt = format!("{col} NOT IN ({query})");
            append_clause(clause, conjunction, &stmt)
        }
        WhereArg::NULL => {
            let stmt = format!("{col} IS NOT NULL");
            append_clause(clause, conjunction, &stmt)
        }
    }
}
