use crate::{data_types::ToSQLData, Error, SQLImplementation, Table};

pub mod implement;
pub mod sql_implementations;

#[derive(Debug)]
pub struct DeleteProps {
    pub connect: SQLImplementation,
    pub table: Table,
    pub clause: Option<String>,
}

pub trait DeleteBuilder {
    /// Builds the DELETE query.
    fn build(self) -> Result<(), Error>;

    /// Adds a WHERE clause to your query.
    /// ```no_run
    /// conn.delete("quarterly_earnings")
    ///     .where_in("quarter", vec!["Q2", "Q3"])
    ///     .build()?;
    /// ```
    fn where_in<T: ToSQLData>(self, column: &str, values: Vec<T>) -> Self;

    /// Adds a WHERE NOT clause to your query.
    /// ```no_run
    /// conn.delete("quarterly_earnings")
    ///     .where_not("quarter", vec!["Q1", "Q4"])
    ///     .build()?;
    /// ```
    fn where_not<T: ToSQLData>(self, column: &str, values: Vec<T>) -> Self;

    /// Deletes where a cell in a column is NULL.
    fn where_null(self, column: &str) -> Self;

    /// Deletes where a cell in column is not NULL.
    fn where_not_null(self, column: &str) -> Self;

    /// Adds a LIKE statement 
    /// ```no_run
    /// conn.delete("inventory") 
    ///     .where_like("product_sku", "%1234%")
    ///     .build()?;
    /// ```
    fn where_like(self, column: &str, value: &str) -> Self;
    
    /// Adds a NOT LIKE statement
    /// ```no_run
    /// conn.delete("inventory") 
    ///     .where_not_like("product_sku", "%5678%")
    ///     .build()?;
    /// ```
    fn where_not_like(self, column: &str, value: &str) -> Self;
}
