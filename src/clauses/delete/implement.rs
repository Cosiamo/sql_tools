use crate::{clauses::where_clause::{utils::where_clause_value_format, WhereDelete}, data_types::ToSQLData, variations::sqlite::delete::sqlite_delete, Error, SQLVariation};

use super::{DeleteBuilder, DeleteProps};

impl DeleteBuilder for DeleteProps {
    fn build(self) -> Result<(), Error> {
        match self.connect {
            SQLVariation::Oracle(_) => todo!(),
            SQLVariation::SQLite(_) => sqlite_delete(self),
        }
    }
    
    fn where_in<T: ToSQLData>(self, column: &str, values: Vec<T>) -> WhereDelete {
        let value = where_clause_value_format(values);
        let where_clause = format!("{} IN ({})", column, value);
        WhereDelete { 
            query_type: self,
            clause: where_clause
        }
    }
    
    fn where_not<T: ToSQLData>(self, column: &str, values: Vec<T>) -> WhereDelete {
        let value = where_clause_value_format(values);
        let where_clause = format!("{} NOT IN ({})", column, value);
        WhereDelete { 
            query_type: self,
            clause: where_clause
        }
    }
    
    fn where_null(self, column: &str) -> WhereDelete {
        let where_clause = format!("{column} IS NULL");
        WhereDelete { 
            query_type: self,
            clause: where_clause
        }
    }
    
    fn where_not_null(self, column: &str) -> WhereDelete {
        let where_clause = format!("{column} IS NOT NULL");
        WhereDelete { 
            query_type: self,
            clause: where_clause
        }
    }
}