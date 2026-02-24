use chrono::NaiveDateTime;
use crate::{
    Error,
    data_types::SQLDataTypes,
    statements::update::SetMatch,
};

pub mod oracle;
pub mod sqlite;

pub(crate) fn build_set_clause(
    set_matches: &[SetMatch],
    date_fmt: impl Fn(&NaiveDateTime) -> String,
    support_query: bool,
) -> Result<String, Error> {
    let len = set_matches.len();
    set_matches
        .iter()
        .enumerate()
        .map(|(idx, set_match)| {
            let val = format_set_value(set_match, &date_fmt, support_query)?;
            Ok(if len == 1 {
                format!("SET {} = {}", set_match.column, val)
            } else if idx == 0 {
                format!("SET {} = {},", set_match.column, val)
            } else if idx == len - 1 {
                format!("{} = {}", set_match.column, val)
            } else {
                format!("{} = {},", set_match.column, val)
            })
        })
        .collect::<Result<Vec<String>, Error>>()
        .map(|v| v.join(" "))
}

fn format_set_value(
    set_match: &SetMatch,
    date_fmt: impl Fn(&NaiveDateTime) -> String,
    support_query: bool, // I need to add set_match.query support to SQLite
) -> Result<String, Error> {
    if support_query && set_match.query {
        return if let SQLDataTypes::Varchar(val) = &set_match.value {
            Ok(format!("({val})"))
        } else {
            Err(Error::UpdateSetQuery)
        };
    }
    Ok(match &set_match.value {
        SQLDataTypes::Varchar(val) => format!("'{}'", val),
        SQLDataTypes::Number(val) => format!("{}", val),
        SQLDataTypes::Float(val) => format!("{}", val),
        SQLDataTypes::Date(val) => date_fmt(val),
        SQLDataTypes::NULL => format!("''"),
    })
}
