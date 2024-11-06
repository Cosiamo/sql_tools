use crate::{data_types::SQLDataTypes, errors::Error, SQLVariation};

pub mod oracle_sql;
pub mod implement;

#[derive(Debug)]
pub struct InsertProps {
    pub connect: SQLVariation,
    pub header: Vec<String>,
    pub grid: Vec<Vec<SQLDataTypes>>,
    pub table: String,
}

#[derive(Debug)]
pub struct InsertPropsFormatted {
    pub insert_props: InsertProps,
}

pub trait InsertBuilder {
    fn format_grid_strings(self) -> Result<InsertPropsFormatted, Error>;
    fn build(self) -> Result<(), Error>;
    fn build_with_progress_bar(self) -> Result<(), Error>;
}