use errors::Error;

pub mod errors;

pub struct OracleConnect {
    connection_string: String,
    username: String,
    password: String
}

impl OracleConnect {
    pub fn new(connection_string: &str, username: &str, password: &str) -> Self {
        Self {
            connection_string: connection_string.to_string(),
            username: username.to_string(),
            password: password.to_string(),
        }
    }
}

pub struct QueryBuilder {
    connect: OracleConnect,
    columns: Vec<String>,
}

pub trait SQLQueryBuilder {
    fn select() {}
}