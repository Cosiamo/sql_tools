use crate::{data_types::ToSQLData, Error, SQLVariation};

use super::where_clause::WhereDelete;

pub mod implement;

#[derive(Debug)]
pub struct DeleteProps {
    pub connect: SQLVariation,
    pub table: String,
    pub clause: Option<String>,
}

pub trait DeleteBuilder {
    fn build(self) -> Result<(), Error>;
    
    fn where_in<T: ToSQLData>(self, column: &str, values: Vec<T>) -> WhereDelete;
    
    fn where_not<T: ToSQLData>(self, column: &str, values: Vec<T>) -> WhereDelete;

    /// Selects where a column is NULL.
    fn where_null(self, column: &str) -> WhereDelete;

    /// Selects where a column is not NULL.
    fn where_not_null(self, column: &str) -> WhereDelete;
}