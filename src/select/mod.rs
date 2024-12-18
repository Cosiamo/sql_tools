use crate::{data_types::{SQLDataTypes, ToSQLData}, errors::Error, where_clause::WhereSelect, SQLVariation};

pub mod oracle_sql;
pub mod implement;

#[derive(Debug)]
pub struct SelectProps {
    pub connect: SQLVariation,
    pub columns: Vec<String>,
    pub table: String,
    pub clause: Option<String>,
}

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

    /// Builds the query. 
    /// This is multi-threaded by default, dividing the number of rows by the number of CPU cores.
    /// If you're using a single core sever, it's recommended to use [`build_single_thread`](`SelectBuilder::build_single_thread`).
    fn build(self) -> Result<Vec<Vec<SQLDataTypes>>, Error>;

    /// Builds the query only using one thread.
    fn build_single_thread(self) -> Result<Vec<Vec<SQLDataTypes>>, Error>;
}