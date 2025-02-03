use crate::{data_types::SQLDataTypes, Error, SQLVariation};

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
    /// Converts unstructured data into [`SQLDataTypes`].
    fn format_grid_strings(self) -> Result<InsertPropsFormatted, Error>;

    /// Builds the query. 
    fn build(self) -> Result<(), Error>;

    /// Builds the query and uses [indicatif](`indicatif::ProgressBar`) to add a progress bar to the terminal.
    fn build_with_progress_bar(self) -> Result<(), Error>;
}