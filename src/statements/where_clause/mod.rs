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
    /// ```no_run
    /// let data: Vec<Vec<SQLDataTypes>> = conn
    ///     .select("users", vec!["first_name", "email"])
    ///     .where_in("continent", vec!["Europe"])
    ///     .and_null("phone_number")
    ///     .build()?;
    /// ```
    /// Is the same as:
    /// ```sql
    /// SELECT first_name, email FROM users
    /// WHERE continent IN ('Europe')
    /// AND phone_number IS NULL;
    /// ```
    fn and_null(self, column: &str) -> Self;

    /// AND IS NOT NULL
    /// ```no_run
    /// let data: Vec<Vec<SQLDataTypes>> = conn
    ///     .select("users", vec!["first_name", "email"])
    ///     .where_in("continent", vec!["Europe"])
    ///     .and_not_null("email")
    ///     .build()?;
    /// ```
    /// Is the same as:
    /// ```sql
    /// SELECT first_name, email FROM users
    /// WHERE continent IN ('Europe')
    /// AND phone_number IS NOT NULL;
    /// ```
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
    /// ```no_run
    /// let data: Vec<Vec<SQLDataTypes>> = conn
    ///     .select("users", vec!["first_name", "email"])
    ///     .where_in("continent", vec!["Europe"])
    ///     .or_null("country")
    ///     .build()?;
    /// ```
    /// Is the same as:
    /// ```sql
    /// SELECT first_name, email FROM users
    /// WHERE continent IN ('Europe')
    /// OR country IS NULL;
    /// ```
    fn or_null(self, column: &str) -> Self;

    /// OR IS NOT NULL
    /// ```no_run
    /// let data: Vec<Vec<SQLDataTypes>> = conn
    ///     .select("users", vec!["first_name", "email"])
    ///     .where_in("continent", vec!["Europe"])
    ///     .or_not_null("email")
    ///     .build()?;
    /// ```
    /// Is the same as:
    /// ```sql
    /// SELECT first_name, email FROM users
    /// WHERE continent IN ('Europe')
    /// OR email IS NOT NULL;
    /// ```
    fn or_not_null(self, column: &str) -> Self;

    /// AND LIKE
    /// ```no_run
    /// let data: Vec<Vec<SQLDataTypes>> = conn
    ///     .select("users", vec!["first_name", "email"])
    ///     .where_in("continent", vec!["Europe"])
    ///     .and_like("country", "Nor%")
    ///     .build()?;
    /// ```
    /// Is the same as:
    /// ```sql
    /// SELECT first_name, email FROM users
    /// WHERE continent IN ('Europe')
    /// AND country LIKE 'Nor%';
    /// ```
    fn and_like(self, column: &str, value: &str) -> Self;

    /// OR LIKE
    /// ```no_run
    /// let data: Vec<Vec<SQLDataTypes>> = conn
    ///     .select("users", vec!["first_name", "email"])
    ///     .where_in("continent", vec!["Europe"])
    ///     .or_like("country", "United%")
    ///     .build()?;
    /// ```
    /// Is the same as:
    /// ```sql
    /// SELECT first_name, email FROM users
    /// WHERE continent IN ('Europe')
    /// OR country LIKE 'United%';
    /// ```
    fn or_like(self, column: &str, value: &str) -> Self;

    /// AND NOT LIKE
    /// ```no_run
    /// let data: Vec<Vec<SQLDataTypes>> = conn
    ///     .select("users", vec!["first_name", "email"])
    ///     .where_in("continent", vec!["Europe"])
    ///     .and_not_like("country", "Nor%")
    ///     .build()?;
    /// ```
    /// Is the same as:
    /// ```sql
    /// SELECT first_name, email FROM users
    /// WHERE continent IN ('Europe')
    /// AND country NOT LIKE 'Nor%';
    /// ```
    fn and_not_like(self, column: &str, value: &str) -> Self;

    /// OR NOT LIKE
    /// ```no_run
    /// let data: Vec<Vec<SQLDataTypes>> = conn
    ///     .select("users", vec!["first_name", "email"])
    ///     .where_in("continent", vec!["Europe"])
    ///     .or_not_like("country", "United%")
    ///     .build()?;
    /// ```
    /// Is the same as:
    /// ```sql
    /// SELECT first_name, email FROM users
    /// WHERE continent IN ('Europe')
    /// OR country NOT LIKE 'United%';
    /// ```
    fn or_not_like(self, column: &str, value: &str) -> Self;
}
