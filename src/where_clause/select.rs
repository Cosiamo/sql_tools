use crate::{data_types::{SQLDataTypes, ToSQLData}, select::{Ordered, SelectBuilder}, Error};

use super::{utils::where_clause_value_format, WhereClauseBuilder, WhereSelect};

impl WhereClauseBuilder for WhereSelect {
    fn and<T: ToSQLData>(self, column: &str, values: Vec<T>) -> Self {
        let value = where_clause_value_format(values);
        let and = format!("{} IN ({})", column, value);
        let clause = format!("{} AND {}", self.clause, and);
        Self { 
            query_type: self.query_type,
            clause,
        }
    }

    fn or<T: ToSQLData>(self, column: &str, values: Vec<T>) -> Self {
        let value = where_clause_value_format(values);
        let or = format!("{} IN ({})", column, value);
        let clause = format!("{} OR {}", self.clause, or);
        Self { 
            query_type: self.query_type,
            clause,
        }
    }
    
    fn and_not<T: ToSQLData>(self, column: &str, values: Vec<T>) -> Self {
        let value = where_clause_value_format(values);
        let and = format!("{} NOT IN ({})", column, value);
        let clause = format!("{} AND {}", self.clause, and);
        Self { 
            query_type: self.query_type,
            clause,
        }
    }
    
    fn or_not<T: ToSQLData>(self, column: &str, values: Vec<T>) -> Self {
        let value = where_clause_value_format(values);
        let or = format!("{} NOT IN ({})", column, value);
        let clause = format!("{} OR {}", self.clause, or);
        Self { 
            query_type: self.query_type,
            clause,
        }
    }
}

impl WhereSelect {
    pub fn order_asc(mut self, column: &str) -> Ordered {
        self.query_type.clause = Some(self.clause);
        self.query_type.order_asc(column)
    }
    
    pub fn order_desc(mut self, column: &str) -> Ordered {
        self.query_type.clause = Some(self.clause);
        self.query_type.order_desc(column)
    }

    pub fn build(mut self) -> Result<Vec<Vec<SQLDataTypes>>, Error> { 
        self.query_type.clause = Some(self.clause);
        self.query_type.build()
    }
    
    pub fn build_single_thread(mut self) -> Result<Vec<Vec<SQLDataTypes>>, Error> {
        self.query_type.clause = Some(self.clause);
        self.query_type.build_single_thread()
    }
}