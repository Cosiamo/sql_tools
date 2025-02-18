use crate::data_types::ToSQLData;

use super::{delete::DeleteProps, select::SelectProps, update::UpdateSet};

pub mod select;
pub mod update;
pub mod utils;
pub mod delete;

/// Contains the SELECT statement properties, [`select::SelectProps`], 
/// as well as the WHERE clause.
#[derive(Debug)]
pub struct WhereSelect {
    pub query_type: SelectProps,
    pub clause: String,
}

/// Contains the UPDATE statement properties, [UpdateSet](`update::UpdateSet`), 
/// as well as the WHERE clause.
#[derive(Debug)]
pub struct WhereUpdate {
    pub query_type: UpdateSet,
    pub clause: String,
}

#[derive(Debug)]
pub struct WhereDelete {
    pub query_type: DeleteProps,
    pub clause: String,
}

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
}