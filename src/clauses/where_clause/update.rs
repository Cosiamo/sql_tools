use crate::{clauses::update::UpdateBuilder, data_types, Error};

use super::{utils::where_clause_value_format, WhereClauseBuilder, WhereUpdate};

impl WhereClauseBuilder for WhereUpdate {
    fn and<T: data_types::ToSQLData>(self, column: &str, values: Vec<T>) -> Self {
        let value = where_clause_value_format(values);
        let and = format!("{} IN ({})", column, value);
        let clause = format!("{} AND {}", self.clause, and);
        Self { 
            query_type: self.query_type,
            clause,
        }
    }

    fn or<T: data_types::ToSQLData>(self, column: &str, values: Vec<T>) -> Self {
        let value = where_clause_value_format(values);
        let or = format!("{} IN ({})", column, value);
        let clause = format!("{} OR {}", self.clause, or);
        Self { 
            query_type: self.query_type,
            clause,
        }
    }
    
    fn and_not<T: data_types::ToSQLData>(self, column: &str, values: Vec<T>) -> Self {
        let value = where_clause_value_format(values);
        let and = format!("{} NOT IN ({})", column, value);
        let clause = format!("{} AND {}", self.clause, and);
        Self { 
            query_type: self.query_type,
            clause,
        }
    }
    
    fn or_not<T: data_types::ToSQLData>(self, column: &str, values: Vec<T>) -> Self {
        let value = where_clause_value_format(values);
        let or = format!("{} NOT IN ({})", column, value);
        let clause = format!("{} OR {}", self.clause, or);
        Self { 
            query_type: self.query_type,
            clause,
        }
    }
    
    fn and_null(self, column: &str) -> Self {
        let and = format!("{column} IS NULL");
        let clause = format!("{} AND {and}", self.clause);
        Self { 
            query_type: self.query_type,
            clause,
        }
    }
    
    fn and_not_null(self, column: &str) -> Self {
        let and = format!("{column} IS NOT NULL");
        let clause = format!("{} AND {and}", self.clause);
        Self { 
            query_type: self.query_type,
            clause,
        }
    }
    
    fn or_null(self, column: &str) -> Self {
        let or = format!("{column} IS NULL");
        let clause = format!("{} AND {or}", self.clause);
        Self { 
            query_type: self.query_type,
            clause,
        }
    }
    
    fn or_not_null(self, column: &str) -> Self {
        let or = format!("{column} IS NOT NULL");
        let clause = format!("{} AND {or}", self.clause);
        Self { 
            query_type: self.query_type,
            clause,
        }
    }
}

impl WhereUpdate {
    pub fn build(mut self) -> Result<(), Error> { 
        self.query_type.clause = Some(self.clause);
        self.query_type.build()
    }

    pub fn build_return_count(mut self) -> Result<usize, Error> { 
        self.query_type.clause = Some(self.clause);
        self.query_type.build_return_count()
    }
}