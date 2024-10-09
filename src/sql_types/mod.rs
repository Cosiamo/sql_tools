use crate::{select::SelectProps, update::UpdateProps, utils::remove_invalid_chars, QueryBuilder, SQLTypes};

#[derive(Debug)]
pub struct OracleConnect {
    pub connection_string: String,
    pub username: String,
    pub password: String,
}

impl OracleConnect {
    pub fn new(connection_string: &str, username: &str, password: &str) -> Self {
        Self {
            connection_string: connection_string.to_string(),
            username: username.to_string(),
            password: password.to_string(),
        }
    }
}

impl QueryBuilder for OracleConnect {
    fn select(self, table: &str, columns: Vec<String>) -> SelectProps {
        let fmt_cols = columns.iter().map(|cols| {
            remove_invalid_chars(cols)
        }).collect::<Vec<String>>();
        SelectProps {
            connect: SQLTypes::Oracle(self),
            columns: fmt_cols,
            table: table.to_string(),
            clause: None,
        }
    }
    
    fn update(self, table: &str) -> UpdateProps {
        UpdateProps {
            connect: SQLTypes::Oracle(self),
            table: table.to_string(),
        }
    }
}