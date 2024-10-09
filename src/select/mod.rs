use crate::{errors::Error, QueryTypes, SQLTypes};

pub mod clause_builder;
pub mod utils;

// ===== Select =====
#[derive(Debug)]
pub struct SelectProps {
    pub connect: SQLTypes,
    pub columns: Vec<String>,
    pub table: String,
    pub clause: Option<String>,
}

pub trait SelectBuilder {
    fn filter(self, column: &str, value: &str) -> WhereClause;
    fn build(self) -> Result<Vec<Vec<Option<String>>>, Error>;
    fn build_single_thread(self) -> Result<Vec<Vec<Option<String>>>, Error>;
}

pub struct WhereClause {
    pub query_type: QueryTypes,
    pub clause: String,
}

pub trait ClauseBuilder {
    fn and(self, column: &str, value: &str) -> Self;
    fn or(self, column: &str, value: &str) -> Self;
    fn build(self) -> Result<Vec<Vec<Option<String>>>, Error>;
    fn build_single_thread(self) -> Result<Vec<Vec<Option<String>>>, Error>;
}