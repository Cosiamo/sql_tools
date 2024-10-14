pub mod oracle_sql;

#[derive(Debug)]
pub struct OracleConnect {
    pub connection_string: String,
    pub username: String,
    pub password: String,
}