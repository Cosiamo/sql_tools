use crate::{errors::Error, select::SelectBuilder};

use super::{ClauseBuilder, WhereSelect};

impl ClauseBuilder for WhereSelect {
    fn and(self, column: &str, value: &str) -> Self {
        let and = format!("{} {}", column, value);
        let clause = format!("{} AND {}", self.clause, and);
        Self { 
            query_type: self.query_type,
            clause,
        }
    }

    fn or(self, column: &str, value: &str) -> Self {
        let or = format!("{} {}", column, value);
        let clause = format!("{} OR {}", self.clause, or);
        Self { 
            query_type: self.query_type,
            clause,
        }
    }
}

impl WhereSelect {
    pub fn build(mut self) -> Result<Vec<Vec<Option<String>>>, Error> { 
        self.query_type.clause = Some(self.clause);
        self.query_type.build()
    }
    
    pub fn build_single_thread(mut self) -> Result<Vec<Vec<Option<String>>>, Error> {
        self.query_type.clause = Some(self.clause);
        self.query_type.build_single_thread()
    }
}