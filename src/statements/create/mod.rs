use crate::{Error, SQLVariation};

pub mod implement;

#[derive(Debug)]
pub struct CreateTable {
    pub connect: SQLVariation,
    pub columns: Vec<CreateColumns>,
    pub table: String,
}

#[derive(Debug)]
pub struct CreateProps {
    pub connect: SQLVariation,
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
    fn add_column(self, column: String, data_type: CreateDataTypes) -> Self;

    /// Builds the query.
    fn build(self) -> Result<(), Error>;
}
