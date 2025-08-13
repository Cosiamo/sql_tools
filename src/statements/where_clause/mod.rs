use crate::data_types::ToSQLData;

pub mod delete;
pub mod select;
pub mod update;
pub mod utils;

/// Trait to add a WHERE clause to a SQL statement.
pub trait WhereClauseBuilder {
    /// Adds an 'AND' to a WHERE clause.
    /// ```no_run
    /// let data: Vec<Vec<SQLDataTypes>> = conn
    ///     .select("users", vec!["first_name", "email"])
    ///     .where_in("continent", vec!["Europe"])
    ///     .and("timezone", vec!["Central European Standard Time", "Eastern European Standard Time"])
    ///     .build()?;
    /// ```
    /// Is the same as:
    /// ```sql
    /// SELECT first_name, email FROM users
    /// WHERE continent IN ('Europe')
    /// AND timezone IN ('Central European Standard Time', 'Eastern European Standard Time');
    /// ```
    fn and<T: ToSQLData>(self, column: &str, values: Vec<T>) -> Self;

    /// AND IS NULL
    fn and_null(self, column: &str) -> Self;

    /// AND IS NOT NULL
    fn and_not_null(self, column: &str) -> Self;

    /// Adds a 'OR' to a WHERE clause.
    /// ```no_run
    /// let data: Vec<Vec<SQLDataTypes>> = conn
    ///     .select("users", vec!["first_name", "email"])
    ///     .where_in("continent", vec!["Europe"])
    ///     .or("country", vec!["United States", "Brazil"])
    ///     .build()?;
    /// ```
    /// Is the same as:
    /// ```sql
    /// SELECT first_name, email FROM users
    /// WHERE continent IN ('Europe')
    /// OR country IN ('United States', 'Brazil');
    /// ```
    fn or<T: ToSQLData>(self, column: &str, values: Vec<T>) -> Self;

    /// Adds an 'AND NOT' to a WHERE clause.
    /// ```no_run
    /// let data: Vec<Vec<SQLDataTypes>> = conn
    ///     .select("users", vec!["first_name", "email"])
    ///     .where_in("continent", vec!["Europe"])
    ///     .and_not("country", vec!["France", "Spain"])
    ///     .build()?;
    /// ```
    /// Is the same as:
    /// ```sql
    /// SELECT first_name, email FROM users
    /// WHERE continent IN ('Europe')
    /// AND country NOT IN ('France', 'Spain');
    /// ```
    fn and_not<T: ToSQLData>(self, column: &str, values: Vec<T>) -> Self;

    /// Adds a 'OR NOT' to a WHERE clause.
    /// ```no_run
    /// let data: Vec<Vec<SQLDataTypes>> = conn
    ///     .select("users", vec!["first_name", "email"])
    ///     .where_not("continent", vec!["Europe"])
    ///     .or_not("country", vec!["United States", "Brazil"])
    ///     .build()?;
    /// ```
    /// Is the same as:
    /// ```sql
    /// SELECT first_name, email FROM users
    /// WHERE continent NOT IN ('Europe')
    /// OR country NOT IN ('United States', 'Brazil');
    /// ```
    fn or_not<T: ToSQLData>(self, column: &str, values: Vec<T>) -> Self;

    /// OR IS NULL
    fn or_null(self, column: &str) -> Self;

    /// OR IS NOT NULL
    fn or_not_null(self, column: &str) -> Self;

    fn and_like(self, column: &str, value: &str) -> Self;

    fn or_like(self, column: &str, value: &str) -> Self;

    fn and_not_like(self, column: &str, value: &str) -> Self;

    fn or_not_like(self, column: &str, value: &str) -> Self;
}
