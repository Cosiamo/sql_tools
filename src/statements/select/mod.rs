use group_by::Grouped;

use crate::{
    Error, SQLVariation,
    data_types::{SQLDataTypes, ToSQLData},
};

pub mod group_by;
pub mod implement;

#[derive(Debug)]
pub struct SelectProps {
    pub connect: SQLVariation,
    pub columns: Vec<String>,
    pub table: Table,
    pub joins: Vec<Joins>,
    pub clause: Option<String>,
    pub order_by: (Option<String>, OrderBy),
    pub group_by: Option<Vec<String>>,
    pub limit: Limit,
}

#[derive(Debug)]
pub struct Table {
    pub name: String,
    pub id: String,
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
    fn inner_join(self, table: Table, primary_column: &str, foreign_column: &str) -> Self;
    
    fn outer_join(self, table: Table, primary_column: &str, foreign_column: &str) -> Self;
    
    fn right_join(self, table: Table, primary_column: &str, foreign_column: &str) -> Self;
    
    fn left_join(self, table: Table, primary_column: &str, foreign_column: &str) -> Self;

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

    /// Order By a column ascending
    fn order_asc(self, column: &str) -> Ordered;

    /// Order By a column descending
    fn order_desc(self, column: &str) -> Ordered;

    /// Group By column(s)
    fn group_by(self, columns: Vec<&str>) -> Grouped;

    fn limit(self, limit: usize, offset: Option<usize>) -> Self;

    /// Builds the query.
    /// This is multi-threaded by default, dividing the number of rows by the number of CPU cores.
    /// If you're using a single core sever, it's recommended to use [`build_single_thread`](`SelectBuilder::build_single_thread`).
    /// [`SQLite`](`SQLVariation::SQLite`) runs better using [`build_single_thread`](`SelectBuilder::build_single_thread`)
    /// (will either fix or remove it as an option in a future update).
    fn build(self) -> Result<Vec<Vec<Box<SQLDataTypes>>>, Error>;

    /// Builds the query only using one thread.
    fn build_single_thread(self) -> Result<Vec<Vec<Box<SQLDataTypes>>>, Error>;
}

impl Table {
    pub fn new(name: &str) -> Self {
        Self {
            name: String::from(name),
            id: crate::utils::generate_id(5),
        }
    }

    pub fn query_fmt(&self) -> String {
        format!("{} {}", self.name, self.id)
    }
}