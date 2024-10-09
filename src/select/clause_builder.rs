use crate::{errors::Error, QueryTypes};

use super::{ClauseBuilder, SelectBuilder, WhereClause};

impl ClauseBuilder for WhereClause {
    fn and(self, column: &str, value: &str) -> Self {
        let and = format!("{} {}", column, value);
        let clause = format!("{} AND {}", self.clause, and);
        WhereClause { 
            query_type: self.query_type,
            clause,
        }
    }

    fn or(self, column: &str, value: &str) -> Self {
        let or = format!("{} {}", column, value);
        let clause = format!("{} OR {}", self.clause, or);
        WhereClause { 
            query_type: self.query_type,
            clause,
        }
    }
    
    fn build(self) -> Result<Vec<Vec<Option<String>>>, Error> { 
        match self.query_type {
            QueryTypes::Select(mut select_props) => {
                select_props.clause = Some(self.clause);
                select_props.build()
            },
            QueryTypes::Update(_update_props) => {
                // update_props.clause = Some(self.clause);
                // update_props.build()
                todo!()
            },
        }
    }
    
    fn build_single_thread(self) -> Result<Vec<Vec<Option<String>>>, Error> {
        match self.query_type {
            QueryTypes::Select(mut select_props) => {
                select_props.clause = Some(self.clause);
                select_props.build_single_thread()
            },
            QueryTypes::Update(_update_props) => {
                // update_props.clause = Some(self.clause);
                // update_props.build_single_thread()
                todo!()
            },
        }
    }
}