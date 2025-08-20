use group_by::Grouped;

use crate::{
    data_types::{SQLDataTypes, ToSQLData}, Error, SQLImplementation, Table
};

pub mod group_by;
pub mod implement;
pub mod sql_implementations;

#[derive(Debug)]
pub struct SelectProps {
    pub connect: SQLImplementation,
    pub columns: Vec<String>,
    pub table: Table,
    pub joins: Vec<Joins>,
    pub clause: Option<String>,
    pub order_by: (Option<String>, OrderBy),
    pub group_by: Option<Vec<String>>,
    pub limit: Limit,
}

#[derive(Debug)]
pub enum OrderBy {
    ASC,
    DESC,
    None,
}

#[derive(Debug)]
pub struct Limit {
    pub limit: Option<usize>,
    pub offset: Option<usize>,
}

#[derive(Debug)]
pub struct Ordered {
    select: SelectProps,
}

#[derive(Debug)]
pub struct Joins {
    pub table: Table,
    pub primary_column: String,
    pub foreign_column: String,
    pub join_type: JoinType,
}

#[derive(Debug)]
pub enum JoinType {
    Inner,
    Outer,
    Right,
    Left,
}

pub trait SelectBuilder {
    /// Inner joins another table to your query.
    ///
    /// The table from the [`select method`](`crate::QueryBuilder::select`) is the primary table and will auto-generate an ID.
    /// The column you want associated with the primary table will be the `primary_column` that's passed into this method and vice versa.
    /// ```no_run
    /// let foreign_table = Table { name: "yearly_earnings".to_owned(), id: "yearly".to_owned() };
    /// let data = conn.select("quarterly_earnings", vec![
    ///         "yearly.year", // Adding the joined table's id to specify which table the column comes from
    ///         "yearly.revenue as yearly_rev",
    ///         "revenue as quarterly_rev", // If you don't add an id, then it will be associated with the primary table
    ///         "profit",
    ///     ])
    ///     .inner_join(foreign_table, "year", "year")
    ///     .where_in("quarter", vec!["Q2", "Q3"])
    ///     .and("yearly.year", vec!["2025", "2026"]) // Same concept from above applies in a WHERE/AND/OR clause
    ///     .build()?;
    /// ```
    fn inner_join(self, foreign_table: Table, primary_column: &str, foreign_column: &str) -> Self;

    /// Outer joins another table to your query.
    ///
    /// The table from the [`select method`](`crate::QueryBuilder::select`) is the primary table and will auto-generate an ID.
    /// The column you want associated with the primary table will be the `primary_column` that's passed into this method and vice versa.
    /// ```no_run
    /// let foreign_table = Table { name: "yearly_earnings".to_owned(), id: "yearly".to_owned() };
    /// let data = conn.select("quarterly_earnings", vec![
    ///         "yearly.year", // Adding the joined table's id to specify which table the column comes from
    ///         "yearly.revenue as yearly_rev",
    ///         "revenue as quarterly_rev", // If you don't add an id, then it will be associated with the primary table
    ///         "profit",
    ///     ])
    ///     .outer_join(foreign_table, "year", "year")
    ///     .where_in("quarter", vec!["Q2", "Q3"])
    ///     .and("yearly.year", vec!["2025", "2026"]) // Same concept from above applies in a WHERE/AND/OR clause
    ///     .build()?;
    /// ```
    fn outer_join(self, foreign_table: Table, primary_column: &str, foreign_column: &str) -> Self;

    /// Right joins another table to your query.
    ///
    /// The table from the [`select method`](`crate::QueryBuilder::select`) is the primary table and will auto-generate an ID.
    /// The column you want associated with the primary table will be the `primary_column` that's passed into this method and vice versa.
    /// ```no_run
    /// let foreign_table = Table { name: "yearly_earnings".to_owned(), id: "yearly".to_owned() };
    /// let data = conn.select("quarterly_earnings", vec![
    ///         "yearly.year", // Adding the joined table's id to specify which table the column comes from
    ///         "yearly.revenue as yearly_rev",
    ///         "revenue as quarterly_rev", // If you don't add an id, then it will be associated with the primary table
    ///         "profit",
    ///     ])
    ///     .right_join(foreign_table, "year", "year")
    ///     .where_in("quarter", vec!["Q2", "Q3"])
    ///     .and("yearly.year", vec!["2025", "2026"]) // Same concept from above applies in a WHERE/AND/OR clause
    ///     .build()?;
    /// ```
    fn right_join(self, foreign_table: Table, primary_column: &str, foreign_column: &str) -> Self;

    /// Left joins another table to your query.
    ///
    /// The table from the [`select method`](`crate::QueryBuilder::select`) is the primary table and will auto-generate an ID.
    /// The column you want associated with the primary table will be the `primary_column` that's passed into this method and vice versa.
    /// ```no_run
    /// let foreign_table = Table { name: "yearly_earnings".to_owned(), id: "yearly".to_owned() };
    /// let data = conn.select("quarterly_earnings", vec![
    ///         "yearly.year", // Adding the joined table's id to specify which table the column comes from
    ///         "yearly.revenue as yearly_rev",
    ///         "revenue as quarterly_rev", // If you don't add an id, then it will be associated with the primary table
    ///         "profit",
    ///     ])
    ///     .left_join(foreign_table, "year", "year")
    ///     .where_in("quarter", vec!["Q2", "Q3"])
    ///     .and("yearly.year", vec!["2025", "2026"]) // Same concept from above applies in a WHERE/AND/OR clause
    ///     .build()?;
    /// ```
    fn left_join(self, foreign_table: Table, primary_column: &str, foreign_column: &str) -> Self;

    /// Adds a WHERE clause to your query.
    /// ```no_run
    /// let data = conn.select("quarterly_earnings", vec!["revenue", "profit"])
    ///     .where_in("quarter", vec!["Q2", "Q3"])
    ///     .build()?;
    /// ```
    fn where_in<T: ToSQLData>(self, column: &str, values: Vec<T>) -> Self;

    /// Adds a WHERE NOT clause to your query.
    /// ```no_run
    /// let data = conn.select("quarterly_earnings", vec!["revenue", "profit"])
    ///     .where_not("quarter", vec!["Q1", "Q4"])
    ///     .build()?;
    /// ```
    fn where_not<T: ToSQLData>(self, column: &str, values: Vec<T>) -> Self;

    /// Selects where a column is NULL.
    fn where_null(self, column: &str) -> Self;

    /// Selects where a column is not NULL.
    fn where_not_null(self, column: &str) -> Self;

    /// Adds a LIKE statement 
    /// ```no_run
    /// let data = conn.select("employees", vec!["name", "position"])
    ///     .where_like("name", "Bob%")
    ///     .build()?;
    /// ```
    /// Is the same as: 
    /// ```sql
    /// SELECT name, position FROM employees WHERE name LIKE 'Bob%';
    /// ```
    fn where_like(self, column: &str, value: &str) -> Self;
    
    /// Adds a NOT LIKE statement
    /// ```no_run
    /// let data = conn.select("employees", vec!["name", "position"])
    ///     .where_not_like("name", "Bob%")
    ///     .build()?;
    /// ```
    /// Is the same as: 
    /// ```sql
    /// SELECT name, position FROM employees WHERE name NOT LIKE 'Bob%';
    /// ```
    fn where_not_like(self, column: &str, value: &str) -> Self;

    /// Order By a column ascending
    fn order_asc(self, column: &str) -> Ordered;

    /// Order By a column descending
    fn order_desc(self, column: &str) -> Ordered;

    /// Group By column(s)
    fn group_by(self, columns: Vec<&str>) -> Grouped;

    fn limit(self, limit: usize, offset: Option<usize>) -> Self;

    /// Builds the query.
    /// This is multi-threaded by default, dividing the number of rows by the number of CPU cores.
    /// If you're using a single core machine, it's recommended to use [`build_single_thread`](`SelectBuilder::build_single_thread`).
    /// [`SQLite`](`SQLImplementation::SQLite`) runs better using [`build_single_thread`](`SelectBuilder::build_single_thread`)
    /// (will either fix or remove it as an option in a future update).
    fn build(self) -> Result<Vec<Vec<Box<SQLDataTypes>>>, Error>;

    /// Builds the query only using one thread.
    fn build_single_thread(self) -> Result<Vec<Vec<Box<SQLDataTypes>>>, Error>;
}