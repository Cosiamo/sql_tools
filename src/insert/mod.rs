use oracle_sql::oracle_build_insert;

use crate::{data_types::FormatData, errors::Error, SQLTypes};

pub mod oracle_sql;

#[derive(Debug)]
pub struct InsertProps<T: FormatData> {
    pub connect: SQLTypes,
    pub grid: Vec<Vec<T>>,
    pub table: String,
}

#[derive(Debug)]
pub struct InsertPropsFormatted<T: FormatData> {
    pub insert_props: InsertProps<T>,
}

impl<T: FormatData> InsertPropsFormatted<T> {
    pub fn build(self) -> Result<(), Error> {
        self.insert_props.build()
    }
}

pub trait InsertBuilder {
    fn format_data<T: FormatData>(self) -> Result<InsertPropsFormatted<T>, Error>;
    fn build(self) -> Result<(), Error>;
}

impl<T: FormatData> InsertBuilder for InsertProps<T> {
    fn format_data<G: FormatData>(self) -> Result<InsertPropsFormatted<G>, Error> {
        todo!()
    }
    
    fn build(self) -> Result<(), Error> {
        match self.connect {
            SQLTypes::Oracle(_) => oracle_build_insert(self),
        }
    }
}