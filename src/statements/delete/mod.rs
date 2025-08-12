use crate::{data_types::ToSQLData, Error, SQLVariation, Table};

pub mod implement;
pub mod sql_implementations;

#[derive(Debug)]
pub struct DeleteProps {
    pub connect: SQLVariation,
    pub table: Table,
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
    fn where_in<T: ToSQLData>(self, column: &str, values: Vec<T>) -> Self;

    /// Adds a WHERE NOT clause to your query.
    /// ```no_run
    /// conn.delete("quarterly_earnings", vec!["revenue", "profit"])
    ///     .where_not("quarter", vec!["Q1", "Q4"])
    ///     .build()?;
    /// ```
    fn where_not<T: ToSQLData>(self, column: &str, values: Vec<T>) -> Self;

    /// Deletes where a cell in a column is NULL.
    fn where_null(self, column: &str) -> Self;

    /// Deletes where a cell in column is not NULL.
    fn where_not_null(self, column: &str) -> Self;

    /// Adds a LIKE statement 
    fn where_like(self, column: &str, value: &str) -> Self;
    
    /// Adds a NOT LIKE statement
    fn where_not_like(self, column: &str, value: &str) -> Self;
}
