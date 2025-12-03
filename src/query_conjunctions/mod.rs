use crate::{data_types::SQLDataTypes, statements::select::ColumnProps};

pub mod implement;
pub(crate) mod utils;

#[derive(Debug)]
/// The argument type for the `where_in`, `where_not`, `and`, `and_not`, `or`, and `or_not` methods.
/// This is split up specifically to prevent SQL injections and to be more intentional with building query structures.
pub enum WhereArg {
    /// A vector of [`SQLDataTypes`](crate::data_types::SQLDataTypes).
    /// [`ToSQLData`](crate::data_types::ToSQLData) is implemented for most of the common data types used in Rust,
    /// so you can easily convert your data to SQLDataTypes using the [`to_sql_fmt()`](crate::data_types::ToSQLData::to_sql_fmt()) method.
    /// ```no_run
    /// let column = ColumnProps{name: "column_name".to_string(), table: "my_table".to_string()};
    /// .where_in(column, WhereArg::Values(vec!["value1".to_sql_fmt(), "value2".to_sql_fmt()]))
    /// ```
    /// Is the equivalent of:
    /// ```sql
    /// WHERE my_table.column_name IN ('value1', 'value2')
    /// ```
    Values(Vec<SQLDataTypes>),
    /// A LIKE clause for string matching.
    /// ```no_run
    /// let column = ColumnProps{name: "column_name".to_string(), table: "my_table".to_string()};
    /// .where_in(column, WhereArg::Like("%value%".to_string()))
    /// ```
    /// Is the equivalent of:
    /// ```sql
    /// WHERE my_table.column_name LIKE '%value%'
    /// ```
    Like(String),
    /// A subquery string to be used in the conjunction statements.
    /// ```no_run
    /// let column = ColumnProps{name: "column_name".to_string(), table: "my_table".to_string()};
    /// .where_in(column, WhereArg::Query("SELECT id FROM other_table WHERE condition".to_string()))
    /// ```
    /// Is the equivalent of:
    /// ```sql
    /// WHERE my_table.column_name IN (SELECT id FROM other_table WHERE condition)
    /// ```
    Query(String),
    /// A NULL value for checking against NULL in the conjunction statements.
    /// ```no_run
    /// let column = ColumnProps{name: "column_name".to_string(), table: "my_table".to_string()};
    /// .where_in(column, WhereArg::NULL)
    /// ```
    /// Is the equivalent of:
    /// ```sql
    /// WHERE my_table.column_name IS NULL
    /// ```
    NULL,
}

/// Trait used for building SQL query conjunctions like WHERE, AND, OR, etc..
pub trait QueryConjunctions {
    /// Adds a `WHERE` clause to your query.
    /// The first argument is the column you want to filter on, and the second argument is the values you want to filter by.
    /// This can be chained with other conjunctions like `and`, `or`, `and_not`, and `or_not`.
    ///
    /// ```sql
    /// WHERE column IN (values);
    /// ```
    fn where_in(self, column: &ColumnProps, values: WhereArg) -> Self;

    /// Adds a `WHERE NOT` clause to your query.
    /// The first argument is the column you want to filter on, and the second argument is the values you want to filter by.
    /// This can be chained with other conjunctions like `and`, `or`, `and_not`, and `or_not`.
    ///
    /// ```sql
    /// WHERE column NOT IN (values);
    /// ```
    fn where_not(self, column: &ColumnProps, values: WhereArg) -> Self;

    /// Adds an `AND` conjunction to a WHERE clause.
    /// The first argument is the column you want to filter on, and the second argument is the values you want to filter by.
    /// This can be chained with other conjunctions like `or`, `and_not`, and `or_not`.
    ///
    /// ```sql
    /// AND column IN (values);
    /// ```
    fn and(self, column: &ColumnProps, values: WhereArg) -> Self;

    /// Adds an `OR` conjunction to a WHERE clause.
    /// The first argument is the column you want to filter on, and the second argument is the values you want to filter by.
    /// This can be chained with other conjunctions like `and`, `and_not`, and `or_not`.
    ///
    /// ```sql
    /// OR column IN (values);
    /// ```
    fn or(self, column: &ColumnProps, values: WhereArg) -> Self;

    /// Adds an `AND NOT` conjunction to a WHERE clause.
    /// The first argument is the column you want to filter on, and the second argument is the values you want to filter by.
    /// This can be chained with other conjunctions like `and`, `or`, and `or_not`.
    ///
    /// ```sql
    /// AND column NOT IN (values);
    /// ```
    fn and_not(self, column: &ColumnProps, values: WhereArg) -> Self;

    /// Adds an `OR NOT` conjunction to a WHERE clause.
    /// The first argument is the column you want to filter on, and the second argument is the values you want to filter by.
    /// This can be chained with other conjunctions like `and`, `and_not`, and `or`.
    ///
    /// ```sql
    /// OR column NOT IN (values);
    /// ```
    fn or_not(self, column: &ColumnProps, values: WhereArg) -> Self;
}
