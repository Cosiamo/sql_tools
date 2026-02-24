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

pub(crate) fn build_where_stmt(column: &ColumnProps, values: WhereArg, negate: bool) -> String {
    let col = format_column(column);
    let not = if negate { "NOT " } else { "" };

    match values {
        WhereArg::Values(items) => {
            let value = where_clause_value_format(items);
            format!("{col} {not}IN ({value})")
        }
        WhereArg::Like(like) => format!("{col} {not}LIKE '{like}'"),
        WhereArg::Query(value) => format!("{col} {not}IN ({value})"),
        WhereArg::NULL => format!("{col} IS {not}NULL"),
    }
}

pub(crate) fn build_conjunction_stmt(
    column: &ColumnProps,
    values: WhereArg,
    clause: &Option<String>,
    conjunction: &str,
    negate: bool,
) -> String {
    let stmt = build_where_stmt(column, values, negate);
    append_clause(clause, conjunction, &stmt)
}
