use crate::{data_types::{SQLDataTypes, ToSQLData}, errors::Error, where_clause::WhereSelect, SQLVariation};

pub mod oracle_sql;
pub mod implement;

#[derive(Debug)]
pub struct SelectProps {
    pub connect: SQLVariation,
    pub columns: Vec<String>,
    pub table: String,
    pub clause: Option<String>,
}

pub trait SelectBuilder {
    fn where_in<T: ToSQLData>(self, column: &str, values: Vec<T>) -> WhereSelect;
    fn where_not<T: ToSQLData>(self, column: &str, values: Vec<T>) -> WhereSelect;
    fn build(self) -> Result<Vec<Vec<SQLDataTypes>>, Error>;
    fn build_single_thread(self) -> Result<Vec<Vec<SQLDataTypes>>, Error>;
}