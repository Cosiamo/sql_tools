use oracle_sql::oracle_build_update;

use crate::{errors::Error, where_clause::WhereUpdate, SQLVariation};

pub mod oracle_sql;

#[derive(Debug)]
pub struct UpdateProps {
    pub connect: SQLVariation,
    pub table: String,
}

#[derive(Debug)]
pub struct UpdateSet {
    pub connect: SQLVariation,
    pub set_match: Vec<SetMatch>,
    pub table: String,
    pub clause: Option<String>,
}

#[derive(Debug)]
pub struct SetMatch {
    pub column: String,
    pub value: String
}

impl UpdateProps {
    pub fn set(self, column: &str, new_value: &str) -> UpdateSet {
        let set = vec![
            SetMatch {
                column: column.to_string(),
                value: new_value.to_string(),
            }
        ];
        UpdateSet {
            connect: self.connect,
            set_match: set,
            table: self.table,
            clause: None,
        }
    }
}

pub trait UpdateBuilder {
    fn set(self, column: &str, new_value: &str) -> Self;
    fn filter(self, column: &str, value: &str) -> WhereUpdate;
    fn build(self) -> Result<(), Error>;
}

impl UpdateBuilder for UpdateSet {
    fn set(mut self, column: &str, new_value: &str) -> Self {
        self.set_match.push(
            SetMatch {
                column: column.to_string(),
                value: new_value.to_string(),
            }
        );
        self
    }

    fn filter(self, column: &str, value: &str) -> WhereUpdate {
        let where_clause = format!("{} {}", column, value);
        WhereUpdate { 
            query_type: self,
            clause: where_clause
        }
    }

    fn build(self) -> Result<(), Error> {
        match self.connect {
            SQLVariation::Oracle(_) => oracle_build_update(self),
        }
    }
}