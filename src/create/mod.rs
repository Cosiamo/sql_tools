use oracle_sql::oracle_build_create_table;

use crate::{errors::Error, SQLVariation};

pub mod oracle_sql;

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
    pub data_type: CreateDataTypes
}

#[derive(Debug, Clone)]
pub enum CreateDataTypes {
    VARCHAR(usize),
    NUMBER,
    FLOAT,
    DATE,
}

pub trait ModifyCreateTable {
    fn add_column(self, column: String, data_type: CreateDataTypes) -> Self;
    fn build(self) -> Result<(), Error>;
}

impl ModifyCreateTable for CreateTable {
    fn add_column(mut self, column: String, data_type: CreateDataTypes) -> Self {
        self.columns.push(CreateColumns{ name: column, data_type });
        self
    }

    fn build(self) -> Result<(), Error> {
        match self.connect {
            SQLVariation::Oracle(_) => oracle_build_create_table(self),
        }
    }
}

impl CreateProps {
    pub fn table(self, table: &str, columns: Vec<CreateColumns>) -> CreateTable {
        CreateTable {
            connect: self.connect,
            columns,
            table: table.to_string(),
        }
    }
}