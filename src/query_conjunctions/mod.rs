use crate::data_types::SQLDataTypes;

pub mod implement;
pub(crate) mod utils;

#[derive(Debug)]
/// The argument type for the `where_in`, `where_not`, `and`, `and_not`, `or`, and `or_not` methods.
/// This is split up specifically to prevent SQL injections and to be more intentional with building query structures.
pub enum WhereArg {
    Values(Vec<SQLDataTypes>),
    Like(String),
    Query(String),
    NULL,
}

/// Trait used for building SQL query conjunctions like WHERE, AND, OR, etc..
pub trait QueryConjunctions {
    /// Adds a `WHERE` clause to your query.
    /// The first argument is the column you want to filter on, and the second argument is the values you want to filter by.
    /// This can be chained with other conjunctions like `and`, `or`, `and_not`, and `or_not`.
    /// 
    /// `.where_in(column, values)`
    ///
    /// ```sql
    /// WHERE column IN (values);
    /// ```
    fn where_in(self, column: &str, values: WhereArg) -> Self;

    /// Adds a `WHERE NOT` clause to your query.
    /// The first argument is the column you want to filter on, and the second argument is the values you want to filter by.
    /// This can be chained with other conjunctions like `and`, `or`, `and_not`, and `or_not`.
    /// 
    /// `.where_not(column, values)`
    ///
    /// ```sql
    /// WHERE column NOT IN (values);
    /// ```
    fn where_not(self, column: &str, values: WhereArg) -> Self;

    /// Adds an `AND` conjunction to a WHERE clause.
    /// The first argument is the column you want to filter on, and the second argument is the values you want to filter by.
    /// This can be chained with other conjunctions like `or`, `and_not`, and `or_not`.
    /// 
    /// `.and(column, values)`
    ///
    /// ```sql
    /// AND column IN (values);
    /// ```
    fn and(self, column: &str, values: WhereArg) -> Self;

    /// Adds an `OR` conjunction to a WHERE clause.
    /// The first argument is the column you want to filter on, and the second argument is the values you want to filter by.
    /// This can be chained with other conjunctions like `and`, `and_not`, and `or_not`.
    /// 
    /// `.or(column, values)`
    ///
    /// ```sql
    /// OR column IN (values);
    /// ```
    fn or(self, column: &str, values: WhereArg) -> Self;

    /// Adds an `AND NOT` conjunction to a WHERE clause.
    /// The first argument is the column you want to filter on, and the second argument is the values you want to filter by.
    /// This can be chained with other conjunctions like `and`, `or`, and `or_not`.
    /// 
    /// `.and_not(column, values)`
    ///
    /// ```sql
    /// AND column NOT IN (values);
    /// ```
    fn and_not(self, column: &str, values: WhereArg) -> Self;

    /// Adds an `OR NOT` conjunction to a WHERE clause.
    /// The first argument is the column you want to filter on, and the second argument is the values you want to filter by.
    /// This can be chained with other conjunctions like `and`, `and_not`, and `or`.
    /// 
    /// `.or_not(column, values)`
    ///
    /// ```sql
    /// OR column NOT IN (values);
    /// ```
    fn or_not(self, column: &str, values: WhereArg) -> Self;
}
