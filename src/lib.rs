#![doc = include_str!("../README.md")]

use clauses::{alter::AlterProps, create::CreateProps, delete::DeleteProps, insert::InsertProps, select::SelectProps, update::UpdateProps};
use data_types::ToSQLData;
use variations::{OracleConnect, SQLiteConnect};

pub mod variations;
pub mod utils;
pub mod data_types;
pub mod clauses;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error(transparent)]
    OracleError(#[from] oracle::Error),
    
    #[error(transparent)]
    SQLiteError(#[from] rusqlite::Error),
    
    #[error("Data does not exists")]
    NoData,

    #[error("Insert is either missing a heading or missing the rows to insert")]
    NoHeading,
    
    #[error("Table doesn't exist")]
    TableDoesNotExist,

    #[error("Could not find number of rows")]
    CountError,
    
    #[error("Wrong connection type passed to function: Contact maintainer")]
    WrongConnectionType,

    #[error("No statement to order by")]
    OrderByError,

    #[error("Incorrect data type returned")]
    SQLDataTypesError,
    
    #[error("Incorrect SQL variation passed to method")]
    SQLVariationError,
    
    #[error("Update using set_query method is not valid")]
    UpdateSetQuery,
}

/// Trait used for the SQL Database types found in [`SQLVariation`] to implement basic SQL queries.
pub trait QueryBuilder {
    /// Creates a new [`SelectProps`] to start building out a select query.
    /// 
    /// For the select method, you add the table you want to select from, then the columns in a vector. 
    /// If you want to select all, simply input `vec!["*"]`. 
    /// You can add a [`where_clause::WhereSelect`] to filter out the rows you want, just like writing a SQL query.
    /// ```no_run
    /// let conn = OracleConnect::new(connection_string, username, password)?;
    /// let data: Vec<Vec<SQLDataTypes>> = conn
    ///     .select("regional_sales", vec!["product_id", "revenue", "sale_date"])
    ///     .where_in("product_id", vec!["1001", "4567"])
    ///     .and_not("city", vec!["Austin", "Dallas"])
    ///     .build()?;
    /// data.iter().for_each(|row: &Vec<SQLDataTypes>| { println!("{:?}", row) });
    /// ```
    /// Is the same as:
    /// ```sql
    /// SELECT product_id, revenue, sale_date FROM regional_sales
    /// WHERE product_id IN ('1001', '4567')
    /// AND city NOT IN ('Austin', 'Dallas');
    /// ```
    fn select(&self, table: &str, columns: Vec<&str>) -> SelectProps;

    /// Creates a new [`UpdateProps`] to start building out an update query.
    /// 
    /// Updates a table's column(s) based on criteria set in the optional [where clauses](#where). 
    /// Updates can return Ok() or the number of rows that were updated.
    /// ```no_run
    /// let conn = OracleConnect::new(connection_string, username, password)?;
    /// conn.update("global_sales")
    ///     .set("continent", "North America")
    ///     .where_in("country", vec!["Canada", "United States", "Mexico"])
    ///     .build()?;
    /// // If you want to get the number of rows that were updated
    /// let count: usize = conn
    ///     .update("global_sales")
    ///     .set("continent", "North America")
    ///     .where_in("country", vec!["Canada", "United States", "Mexico"])
    ///     .build_return_count()?;
    /// ```
    fn update(&self, table: &str) -> UpdateProps;

    /// Creates a new [`InsertProps`] to start building out an insert query.
    /// 
    /// Inserts a grid (two-dimensional vector) of data into your database. 
    /// Can take any type that has the [`ToSQLData`] trait implemented. 
    /// If the table does not exist, it will automatically create a new table (will have an abort option in a future update).
    /// The first row should be the header.
    /// ```no_run
    /// let conn = OracleConnect::new(connection_string, username, password)?;
    /// let data: Vec<Vec<&str>> = vec![
    ///     vec!["Column_A", "Column_B", "Column_C"],
    ///     vec!["a1", "b1", "c1"],
    ///     vec!["a2", "b2", "c2"],
    ///     vec!["a3", "b3", "c3"],
    /// ];
    /// conn.insert("my_table", data)?.build()?;
    /// ```
    fn insert<T: ToSQLData>(&self, table: &str, data: Vec<Vec<T>>) -> Result<InsertProps, Error>;

    /// Creates a new [`CreateProps`] to start building a create query.
    /// 
    /// Creates a table using a vector of the `CreateColumns` struct and the `CreateDataTypes` to apply the correct types to the new columns.
    /// ```no_run
    /// let conn = OracleConnect::new(connection_string, username, password)?;
    /// let columns = vec![
    ///     CreateColumns{ name: "Column_A".to_string(), data_type: CreateDataTypes::VARCHAR(20 as usize) },
    ///     CreateColumns{ name: "Column_B".to_string(), data_type: CreateDataTypes::NUMBER },
    ///     CreateColumns{ name: "Column_C".to_string(), data_type: CreateDataTypes::FLOAT },
    /// ];
    ///
    /// conn.create()
    ///     .table("my_table", columns)
    ///     .build()?;
    /// ```
    ///
    /// You can add a column after you initiated the create table process.
    /// ```no_run
    /// let my_table: CreateTable = conn.create()
    ///     .table("my_table", columns);
    ///
    /// if add_date_col == true {
    ///     my_table.add_column("Column_D".to_string(), CreateDataTypes::DATE);
    /// }
    ///
    /// my_table.build()?;
    /// ```
    fn create(&self) -> CreateProps;

    /// Creates a new [`AlterProps`] to start the process of altering a table or column(s).
    /// 
    /// ```no_run
    /// let conn = OracleConnect::new(connection_string, username, password)?;
    /// conn.alter()
    ///     .table("local_sales")
    ///     .rename("regional_sales")
    ///     .build()?;
    /// ```
    fn alter(&self) -> AlterProps;

    /// A Delete statement
    fn delete(&self, table: &str) -> DeleteProps;
}

#[derive(Debug)]
/// The various types of SQL connections 
pub enum SQLVariation {
    Oracle(OracleConnect),
    SQLite(SQLiteConnect),
}