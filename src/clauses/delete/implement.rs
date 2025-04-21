use crate::{clauses::where_clause::utils::where_clause_value_format, data_types::ToSQLData, variations::{oracle::delete::oracle_build_delete, sqlite::delete::sqlite_delete}, Error, SQLVariation};

use super::{DeleteBuilder, DeleteProps};

impl DeleteBuilder for DeleteProps {
    fn build(self) -> Result<(), Error> {
        match self.connect {
            SQLVariation::Oracle(_) => oracle_build_delete(self),
            SQLVariation::SQLite(_) => sqlite_delete(self),
        }
    }
    
    fn where_in<T: ToSQLData>(mut self, column: &str, values: Vec<T>) -> Self {
        let value = where_clause_value_format(values);
        let where_clause = format!("{column} IN ({value})");
        self.clause = Some(where_clause);
        self
    }
    
    fn where_not<T: ToSQLData>(mut self, column: &str, values: Vec<T>) -> Self {
        let value = where_clause_value_format(values);
        let where_clause = format!("{column} NOT IN ({value})");
        self.clause = Some(where_clause);
        self
    }
    
    fn where_null(mut self, column: &str) -> Self {
        let where_clause = format!("{column} IS NULL");
        self.clause = Some(where_clause);
        self
    }
    
    fn where_not_null(mut self, column: &str) -> Self {
        let where_clause = format!("{column} IS NOT NULL");
        self.clause = Some(where_clause);
        self
    }
}