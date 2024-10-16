#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error(transparent)]
    OracleError(#[from] oracle::Error),

    #[error("Could not find number of rows")]
    CountError,
    
    #[error("Wrong connection type passed to function: Contact maintainer")]
    WrongConnectionType,
}