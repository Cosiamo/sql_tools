use oracle_sql::OracleConnect;
use select::SelectProps;
use update::UpdateProps;

pub mod errors;
pub mod oracle_sql;
pub mod select;
pub mod update;
pub mod utils;

pub trait QueryBuilder {
    fn select(self, table: &str, columns: Vec<String>) -> SelectProps;
}

#[derive(Debug)]
pub enum SQLTypes {
    Oracle(OracleConnect),
}

pub enum QueryTypes {
    Select(SelectProps),
    Update(UpdateProps),
}