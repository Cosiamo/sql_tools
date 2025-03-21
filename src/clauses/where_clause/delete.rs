use crate::{clauses::delete::DeleteBuilder, data_types::ToSQLData, Error};

use super::{utils::where_clause_value_format, WhereClauseBuilder, WhereDelete};

impl WhereClauseBuilder for WhereDelete {
    fn and<T: ToSQLData>(self, column: &str, values: Vec<T>) -> Self {
        let value = where_clause_value_format(values);
        let and = format!("{column} IN ({value})");
        let clause = format!("{} AND {}", self.clause, and);
        Self { 
            query_type: self.query_type,
            clause,
        }
    }

    fn or<T: ToSQLData>(self, column: &str, values: Vec<T>) -> Self {
        let value = where_clause_value_format(values);
        let or = format!("{column} IN ({value})");
        let clause = format!("{} OR {}", self.clause, or);
        Self { 
            query_type: self.query_type,
            clause,
        }
    }
    
    fn and_not<T: ToSQLData>(self, column: &str, values: Vec<T>) -> Self {
        let value = where_clause_value_format(values);
        let and = format!("{column} NOT IN ({value})");
        let clause = format!("{} AND {}", self.clause, and);
        Self { 
            query_type: self.query_type,
            clause,
        }
    }
    
    fn or_not<T: ToSQLData>(self, column: &str, values: Vec<T>) -> Self {
        let value = where_clause_value_format(values);
        let or = format!("{column} NOT IN ({value})");
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

impl WhereDelete {
    pub fn build(mut self) -> Result<(), Error> { 
        self.query_type.clause = Some(self.clause);
        self.query_type.build()
    }
}