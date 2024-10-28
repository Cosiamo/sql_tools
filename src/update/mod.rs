use oracle_sql::oracle_build_update;

use crate::{data_types::{SQLDataTypes, ToSQLData}, errors::Error, where_clause::{utils::where_clause_value_format, WhereUpdate}, SQLVariation};

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
    pub value: SQLDataTypes
}

impl UpdateProps {
    pub fn set<T: ToSQLData>(self, column: &str, new_value: T) -> UpdateSet {
        let set = vec![
            SetMatch {
                column: column.to_string(),
                value: new_value.fmt_data(),
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
    fn set<T: ToSQLData>(self, column: &str, new_value: T) -> Self;
    fn where_in<T: ToSQLData>(self, column: &str, values: Vec<T>) -> WhereUpdate;
    fn where_not<T: ToSQLData>(self, column: &str, values: Vec<T>) -> WhereUpdate;
    fn build(self) -> Result<(), Error>;
}

impl UpdateBuilder for UpdateSet {
    fn set<T: ToSQLData>(mut self, column: &str, new_value: T) -> Self {
        self.set_match.push(
            SetMatch {
                column: column.to_string(),
                value: new_value.fmt_data(),
            }
        );
        self
    }

    fn where_in<T: ToSQLData>(self, column: &str, values: Vec<T>) -> WhereUpdate {
        let value = where_clause_value_format(values);
        let where_clause = format!("{} IN ({})", column, value);
        WhereUpdate { 
            query_type: self,
            clause: where_clause
        }
    }

    fn where_not<T: ToSQLData>(self, column: &str, values: Vec<T>) -> WhereUpdate {
        let value = where_clause_value_format(values);
        let where_clause = format!("{} NOT IN ({})", column, value);
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