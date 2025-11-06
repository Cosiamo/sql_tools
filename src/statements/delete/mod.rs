use crate::{Error, SQLImplementation};

pub mod implement;
pub mod sql_implementations;

#[derive(Debug)]
pub struct DeleteProps {
    pub connect: SQLImplementation,
    pub table: String,
    pub clause: Option<String>,
}

pub trait DeleteBuilder {
    /// Builds the DELETE query.
    fn build(self) -> Result<(), Error>;
}
