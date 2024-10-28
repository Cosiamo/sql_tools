#![doc = include_str!("../README.md")]

use create::CreateProps;
use data_types::ToSQLData;
use insert::InsertProps;
use sql_variations::OracleConnect;
use select::SelectProps;
use update::UpdateProps;

pub mod errors;
pub mod sql_variations;
pub mod select;
pub mod update;
pub mod utils;
pub mod where_clause;
pub mod insert;
pub mod data_types;
pub mod create;

/// Trait used for the SQL Database types found in [`SQLVariation`] to implement basic SQL queries.
pub trait QueryBuilder {
    /// Creates a new [`SelectProps`] to start building out a select query.
    /// 
    /// To make a simple select statement, you need to input the table or view name, 
    /// as well as a vector containing the column names.
    fn select(&self, table: &str, columns: Vec<&str>) -> SelectProps;
    fn update(&self, table: &str) -> UpdateProps;
    fn insert<T: ToSQLData>(&self, table: &str, data: Vec<Vec<T>>) -> InsertProps;
    fn create(&self) -> CreateProps;
}

#[derive(Debug)]
pub enum SQLVariation {
    Oracle(OracleConnect),
}