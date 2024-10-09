use oracle_sql::{oracle_build_select, oracle_build_single_thread_select};

use crate::{errors::Error, where_clause::WhereSelect, SQLTypes};

pub mod oracle_sql;

#[derive(Debug)]
pub struct SelectProps {
    pub connect: SQLTypes,
    pub columns: Vec<String>,
    pub table: String,
    pub clause: Option<String>,
}

pub trait SelectBuilder {
    fn filter(self, column: &str, value: &str) -> WhereSelect;
    fn build(self) -> Result<Vec<Vec<Option<String>>>, Error>;
    fn build_single_thread(self) -> Result<Vec<Vec<Option<String>>>, Error>;
}

impl SelectBuilder for SelectProps {
    fn filter(self, column: &str, value: &str) -> WhereSelect {
        let where_clause = format!("{} {}", column, value);
        WhereSelect { 
            query_type: self,
            clause: where_clause
        }
    }

    fn build(self) -> Result<Vec<Vec<Option<String>>>, Error> {
        match self.connect {
            SQLTypes::Oracle(_) => oracle_build_select(self),
        }
    }
    
    fn build_single_thread(self) -> Result<Vec<Vec<Option<String>>>, Error> {
        match self.connect {
            SQLTypes::Oracle(_) => oracle_build_single_thread_select(self),
        }
    }
}