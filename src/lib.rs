use sql_types::OracleConnect;
use select::SelectProps;
use update::UpdateProps;

pub mod errors;
pub mod sql_types;
pub mod select;
pub mod update;
pub mod utils;
pub mod where_clause;

pub trait QueryBuilder {
    fn select(self, table: &str, columns: Vec<String>) -> SelectProps;
    fn update(self, table: &str) -> UpdateProps;
}

#[derive(Debug)]
pub enum SQLTypes {
    Oracle(OracleConnect),
}

pub enum QueryTypes {
    Select(SelectProps),
    Update(UpdateProps),
}