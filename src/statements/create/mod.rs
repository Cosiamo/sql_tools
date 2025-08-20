use crate::{Error, SQLImplementation};

pub mod implement;
pub mod sql_implementations;

#[derive(Debug, Clone)]
pub struct CreateTable {
    pub connect: SQLImplementation,
    pub columns: Vec<CreateColumns>,
    pub table: String,
}

#[derive(Debug)]
pub struct CreateProps {
    pub connect: SQLImplementation,
}

#[derive(Debug, Clone)]
pub struct CreateColumns {
    pub name: String,
    pub data_type: CreateDataTypes,
}

#[derive(Debug, Clone)]
pub enum CreateDataTypes {
    VARCHAR(usize),
    NUMBER,
    FLOAT,
    DATE,
}

pub trait ModifyCreateTable {
    /// Adds a column to the CREATE TABLE query.
    fn add_column(&mut self, column: String, data_type: CreateDataTypes) -> Self;

    /// Builds the query.
    fn build(self) -> Result<(), Error>;
}
