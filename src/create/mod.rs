use oracle_sql::oracle_build_create_table;

use crate::{errors::Error, SQLTypes};

pub mod oracle_sql;

#[derive(Debug)]
pub struct CreateProps {
    pub connect: SQLTypes,
    pub columns: Vec<CreateColumns>,
    pub table: String,
}

#[derive(Debug)]
pub struct CreateColumns {
    pub name: String,
    pub data_type: CreateDataTypes
}

#[derive(Debug)]
pub enum CreateDataTypes {
    VARCHAR(usize),
    INT,
    FLOAT,
    DATE,
}

pub trait CreateTable {
    fn add_column(self, column: String, data_type: CreateDataTypes) -> Self;
    fn build(self) -> Result<(), Error>;
}

impl CreateTable for CreateProps {
    fn add_column(mut self, column: String, data_type: CreateDataTypes) -> Self {
        self.columns.push(CreateColumns{ name: column, data_type });
        self
    }

    fn build(self) -> Result<(), Error> {
        match self.connect {
            SQLTypes::Oracle(_) => oracle_build_create_table(self),
        }
    }
}