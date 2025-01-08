use group_by::Grouped;

use crate::{data_types::{SQLDataTypes, ToSQLData}, Error, where_clause::WhereSelect, SQLVariation};

pub mod oracle_sql;
pub mod implement;
pub mod group_by;

#[derive(Debug)]
pub struct SelectProps {
    pub connect: SQLVariation,
    pub columns: Vec<String>,
    pub table: String,
    pub clause: Option<String>,
    pub order_by: (Option<String>, OrderBy),
    pub group_by: Option<Vec<String>>,
}

#[derive(Debug)]
pub enum OrderBy {
    ASC,
    DESC,
    None,
}

#[derive(Debug)]
pub struct Ordered { select: SelectProps }

pub trait SelectBuilder {
    /// Adds a WHERE clause to your query.
    /// ```no_run
    /// let data = conn.select("quarterly_earnings", vec!["revenue", "profit"])
    ///     .where_in("quarter", vec!["Q2", "Q3"])
    ///     .build()?;
    /// ```
    fn where_in<T: ToSQLData>(self, column: &str, values: Vec<T>) -> WhereSelect;
    
    /// Adds a WHERE NOT clause to your query.
    /// ```no_run
    /// let data = conn.select("quarterly_earnings", vec!["revenue", "profit"])
    ///     .where_not("quarter", vec!["Q1", "Q4"])
    ///     .build()?;
    /// ```
    fn where_not<T: ToSQLData>(self, column: &str, values: Vec<T>) -> WhereSelect;

    /// Selects where a column is NULL.
    fn where_null(self, column: &str) -> WhereSelect;

    /// Selects where a column is not NULL.
    fn where_not_null(self, column: &str) -> WhereSelect;

    /// Order By a column ascending
    fn order_asc(self, column: &str) -> Ordered;

    /// Order By a column descending
    fn order_desc(self, column: &str) -> Ordered;

    /// Group By column(s)
    fn group_by(self, columns: Vec<&str>) -> Grouped;

    /// Builds the query. 
    /// This is multi-threaded by default, dividing the number of rows by the number of CPU cores.
    /// If you're using a single core sever, it's recommended to use [`build_single_thread`](`SelectBuilder::build_single_thread`).
    fn build(self) -> Result<Vec<Vec<SQLDataTypes>>, Error>;

    /// Builds the query only using one thread.
    fn build_single_thread(self) -> Result<Vec<Vec<SQLDataTypes>>, Error>;
}