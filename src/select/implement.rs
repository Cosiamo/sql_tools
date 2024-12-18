use crate::{data_types::{SQLDataTypes, ToSQLData}, errors::Error, where_clause::{utils::where_clause_value_format, WhereSelect}, SQLVariation};

use super::{oracle_sql::{oracle_build_select, oracle_build_single_thread_select}, SelectBuilder, SelectProps};

impl SelectBuilder for SelectProps {
    fn where_in<T: ToSQLData>(self, column: &str, values: Vec<T>) -> WhereSelect {
        let value = where_clause_value_format(values);
        let where_clause = format!("{} IN ({})", column, value);
        WhereSelect { 
            query_type: self,
            clause: where_clause
        }
    }

    fn where_not<T: ToSQLData>(self, column: &str, values: Vec<T>) -> WhereSelect {
        let value = where_clause_value_format(values);
        let where_clause = format!("{} NOT IN ({})", column, value);
        WhereSelect { 
            query_type: self,
            clause: where_clause
        }
    }

    fn build(self) -> Result<Vec<Vec<SQLDataTypes>>, Error> {
        match self.connect {
            SQLVariation::Oracle(_) => oracle_build_select(self),
        }
    }
    
    fn build_single_thread(self) -> Result<Vec<Vec<SQLDataTypes>>, Error> {
        match self.connect {
            SQLVariation::Oracle(_) => oracle_build_single_thread_select(self),
        }
    }
}