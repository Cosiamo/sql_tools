use crate::{errors::Error, update::UpdateBuilder};

use super::{ClauseBuilder, WhereUpdate};

impl ClauseBuilder for WhereUpdate {
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

impl WhereUpdate {
    pub fn build(mut self) -> Result<(), Error> { 
        self.query_type.clause = Some(self.clause);
        self.query_type.build()
    }
}