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
    
    /// Adds a WHERE clause to your query.
    /// ```no_run
    /// conn.delete("quarterly_earnings", vec!["revenue", "profit"])
    ///     .where_in("quarter", vec!["Q2", "Q3"])
    ///     .build()?;
    /// ```
    fn where_in<T: ToSQLData>(self, column: &str, values: Vec<T>) -> WhereDelete;
    
    /// Adds a WHERE NOT clause to your query.
    /// ```no_run
    /// conn.delete("quarterly_earnings", vec!["revenue", "profit"])
    ///     .where_not("quarter", vec!["Q1", "Q4"])
    ///     .build()?;
    /// ```
    fn where_not<T: ToSQLData>(self, column: &str, values: Vec<T>) -> WhereDelete;

    /// Deletes where a cell in a column is NULL.
    fn where_null(self, column: &str) -> WhereDelete;

    /// Deletes where a cell in column is not NULL.
    fn where_not_null(self, column: &str) -> WhereDelete;
}