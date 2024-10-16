use oracle_sql::oracle_build_insert;

use crate::{data_types::SQLDataTypes, errors::Error, SQLTypes};

pub mod oracle_sql;

#[derive(Debug)]
pub struct InsertProps {
    pub connect: SQLTypes,
    pub header: Vec<String>,
    pub grid: Vec<Vec<SQLDataTypes>>,
    pub table: String,
}

#[derive(Debug)]
pub struct InsertPropsFormatted {
    pub insert_props: InsertProps,
}

impl InsertPropsFormatted {
    pub fn build(self) -> Result<(), Error> {
        self.insert_props.build()
    }
}

pub trait InsertBuilder {
    fn format_data(self) -> Result<InsertPropsFormatted, Error>;
    fn build(self) -> Result<(), Error>;
}

impl InsertBuilder for InsertProps {
    fn format_data(self) -> Result<InsertPropsFormatted, Error> {
        todo!()
    }
    
    fn build(self) -> Result<(), Error> {
        match self.connect {
            SQLTypes::Oracle(_) => oracle_build_insert(self),
        }
    }
}