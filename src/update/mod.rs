use crate::{data_types::{SQLDataTypes, ToSQLData}, errors::Error, where_clause::WhereUpdate, SQLVariation};

pub mod oracle_sql;
pub mod implement;

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

pub trait UpdateBuilder {
    fn set<T: ToSQLData>(self, column: &str, new_value: T) -> Self;
    fn where_in<T: ToSQLData>(self, column: &str, values: Vec<T>) -> WhereUpdate;
    fn where_not<T: ToSQLData>(self, column: &str, values: Vec<T>) -> WhereUpdate;
    fn build(self) -> Result<(), Error>;
    fn build_return_count(self) -> Result<usize, Error>;
}
