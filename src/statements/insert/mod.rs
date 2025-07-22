use crate::{Error, SQLVariation, data_types::SQLDataTypes};

pub mod data_conversion;
pub mod implement;

#[derive(Debug)]
pub struct InsertProps {
    pub connect: SQLVariation,
    pub header: Vec<String>,
    pub grid: Vec<Vec<SQLDataTypes>>,
    pub table: String,
    pub create: bool,
}

#[derive(Debug, Clone)]
pub struct DatatypeIndices {
    pub is_varchar: Vec<usize>,
    pub is_float: Vec<usize>,
    pub is_int: Vec<usize>,
    pub is_date: Vec<usize>,
}

pub struct InsertPropsFormatted {
    pub insert_props: InsertProps,
}

pub trait InsertBuilder {
    /// Converts unstructured data into [`SQLDataTypes`].
    fn format_grid_strings(self) -> Result<InsertPropsFormatted, Error>;

    /// Will Create the input table if it does not exist.
    fn create_table(self) -> Self;

    /// Builds the query.
    fn build(self) -> Result<(), Error>;

    /// Builds the query and uses [indicatif](`indicatif::ProgressBar`) to add a progress bar to the terminal.
    fn build_with_progress_bar(self) -> Result<(), Error>;
}
