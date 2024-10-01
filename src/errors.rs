#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error(transparent)]
    OracleError(#[from] oracle::Error),
}