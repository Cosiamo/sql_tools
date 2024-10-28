pub mod oracle_sql;

#[derive(Debug, Clone)]
pub struct OracleConnect {
    pub connection_string: String,
    pub username: String,
    pub password: String,
}