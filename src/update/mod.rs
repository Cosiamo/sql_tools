use crate::{data_types::{SQLDataTypes, ToSQLData}, Error, where_clause::WhereUpdate, SQLVariation};

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

    /// Adds a WHERE clause to your query.
    /// ```no_run
    /// conn.update("test_grades")
    ///     .set("passed", "false")
    ///     .where_in("name", vec!["John Doe", "Jane Smith"])
    ///     .build()?;
    /// ```
    /// Is the same as:
    /// ```sql
    /// UPDATE test_grades
    /// SET passed = 'false'
    /// WHERE name NOT IN ('John Doe');
    /// ```
    fn where_in<T: ToSQLData>(self, column: &str, values: Vec<T>) -> WhereUpdate;
    
    /// Adds a WHERE NOT clause to your query.
    /// ```no_run
    /// conn.update("test_grades")
    ///     .set("passed", "true")
    ///     .where_not("name", vec!["John Doe", "Jane Smith"])
    ///     .build()?;
    /// ```
    /// Is the same as:
    /// ```sql
    /// UPDATE test_grades
    /// SET passed = 'true'
    /// WHERE name NOT IN ('John Doe');
    /// ```
    fn where_not<T: ToSQLData>(self, column: &str, values: Vec<T>) -> WhereUpdate;

    /// Builds the query.
    fn build(self) -> Result<(), Error>;

    /// Builds the query and returns the number of row updated.
    fn build_return_count(self) -> Result<usize, Error>;
}
