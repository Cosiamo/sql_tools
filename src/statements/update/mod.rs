use crate::{
    Error, SQLImplementation,
    data_types::{SQLDataTypes, ToSQLData},
};

pub mod implement;
pub mod sql_implementations;

#[derive(Debug)]
pub struct UpdateInitialization {
    pub connect: SQLImplementation,
    pub table: String,
}

#[derive(Debug)]
pub struct UpdateProps {
    pub connect: SQLImplementation,
    pub set_match: Vec<SetMatch>,
    pub table: String,
    pub clause: Option<String>,
}

#[derive(Debug)]
pub struct SetMatch {
    pub column: String,
    pub value: SQLDataTypes,
    pub query: bool,
}

pub trait UpdateBuilder {
    /// Sets a column to a new value.
    /// Implements the [`ToSQLData`] trait for new values.
    /// For UPDATEs, it's highly recommended to add a [`where_in`](UpdateBuilder::where_in) or [`where_not`](UpdateBuilder::where_not)
    /// (unless you want to update all entries).
    /// ```no_run
    /// conn.update("quarterly_earnings")
    ///     .set("predicted_earnings", 1000000)
    ///     .build()?;
    /// ```
    fn set<T: ToSQLData>(self, column: &str, new_value: T) -> Self;

    /// Sets a column equal to the result of a SELECT query.
    fn set_query(self, column: &str, query: &str) -> Self;

    /// Builds the query.
    fn build(self) -> Result<(), Error>;

    /// Builds the query and returns the number of row updated.
    fn build_return_count(self) -> Result<usize, Error>;
}
