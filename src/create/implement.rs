use crate::{Error, SQLVariation};

use super::{oracle_sql::oracle_build_create_table, sqlite::sqlite_build_create_table, CreateColumns, CreateDataTypes, CreateProps, CreateTable, ModifyCreateTable};

impl CreateProps {
    pub fn table(self, table: &str, columns: Vec<CreateColumns>) -> CreateTable {
        CreateTable {
            connect: self.connect,
            columns,
            table: table.to_string(),
        }
    }
}

impl ModifyCreateTable for CreateTable {
    fn add_column(mut self, column: String, data_type: CreateDataTypes) -> Self {
        self.columns.push(CreateColumns{ name: column, data_type });
        self
    }

    fn build(self) -> Result<(), Error> {
        match self.connect {
            SQLVariation::Oracle(_) => oracle_build_create_table(self),
            SQLVariation::SQLite(_) => sqlite_build_create_table(self),
        }
    }
}