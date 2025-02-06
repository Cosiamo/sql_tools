pub mod oracle;
pub mod sqlite;
pub mod utils;

#[derive(Debug, Clone)]
pub struct OracleConnect {
    pub connection_string: String,
    pub username: String,
    pub password: String,
}

#[derive(Debug, Clone)]
pub struct SQLiteConnect {
    pub path: String,
}