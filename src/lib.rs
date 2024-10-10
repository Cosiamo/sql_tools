use data_types::FormatData;
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

pub trait QueryBuilder {
    fn select(self, table: &str, columns: Vec<String>) -> SelectProps;
    fn update(self, table: &str) -> UpdateProps;
    fn insert<T: FormatData>(self, table: &str, data: Vec<Vec<T>>) -> InsertProps<T>;
}

#[derive(Debug)]
pub enum SQLTypes {
    Oracle(OracleConnect),
}

pub enum QueryTypes<T: FormatData> {
    Select(SelectProps),
    Update(UpdateProps),
    Insert(InsertProps<T>),
}