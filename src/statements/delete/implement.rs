use crate::{
    Error, SQLImplementation, data_types::ToSQLData, query_conjunctions::utils::{match_table_ids, where_clause_value_format}, statements::delete::sql_implementations::{oracle::oracle_build_delete, sqlite::sqlite_delete}
};

use super::{DeleteBuilder, DeleteProps};

impl DeleteBuilder for DeleteProps {
    fn build(self) -> Result<(), Error> {
        match self.connect {
            SQLImplementation::Oracle(_) => oracle_build_delete(self),
            SQLImplementation::SQLite(_) => sqlite_delete(self),
        }
    }

    fn where_in<T: ToSQLData>(mut self, column: &str, values: Vec<T>) -> Self {
        let column = match_table_ids(&self.table, column);
        let value = where_clause_value_format(values);
        let where_clause = format!("{column} IN ({value})");
        self.clause = Some(where_clause);
        self
    }

    fn where_not<T: ToSQLData>(mut self, column: &str, values: Vec<T>) -> Self {
        let column = match_table_ids(&self.table, column);
        let value = where_clause_value_format(values);
        let where_clause = format!("{column} NOT IN ({value})");
        self.clause = Some(where_clause);
        self
    }

    fn where_null(mut self, column: &str) -> Self {
        let column = match_table_ids(&self.table, column);
        let where_clause = format!("{column} IS NULL");
        self.clause = Some(where_clause);
        self
    }

    fn where_not_null(mut self, column: &str) -> Self {
        let column = match_table_ids(&self.table, column);
        let where_clause = format!("{column} IS NOT NULL");
        self.clause = Some(where_clause);
        self
    }
    
    fn where_like(mut self, column: &str, value: &str) -> Self {
        let column = match_table_ids(&self.table, column);
        let like = format!("{column} LIKE '{value}'");
        self.clause = Some(like);
        self
    }
    
    fn where_not_like(mut self, column: &str, value: &str) -> Self {
        let column = match_table_ids(&self.table, column);
        let like = format!("{column} NOT LIKE '{value}'");
        self.clause = Some(like);
        self
    }
}
