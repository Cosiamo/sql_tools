use crate::{data_types::{SQLDataTypes, ToSQLData}, Error, where_clause::{utils::where_clause_value_format, WhereSelect}, SQLVariation};

use super::{group_by::Grouped, oracle_sql::{oracle_build_select, oracle_build_single_thread_select}, OrderBy, Ordered, SelectBuilder, SelectProps};

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
    
    fn where_null(self, column: &str) -> WhereSelect {
        let where_clause = format!("{column} IS NULL");
        WhereSelect { 
            query_type: self,
            clause: where_clause
        }
    }
    
    fn where_not_null(self, column: &str) -> WhereSelect {
        let where_clause = format!("{column} IS NOT NULL");
        WhereSelect { 
            query_type: self,
            clause: where_clause
        }
    }
    
    fn order_asc(mut self, column: &str) -> Ordered {
        self.order_by = (Some(column.to_string()), OrderBy::ASC);
        Ordered { select: self }
    }
    
    fn order_desc(mut self, column: &str) -> Ordered  {
        self.order_by = (Some(column.to_string()), OrderBy::DESC);
        Ordered { select: self }
    }

    fn group_by(mut self, columns: Vec<&str>) -> Grouped {
        self.group_by = Some(columns.iter().map(|col| col.to_string()).collect::<Vec<String>>());
        Grouped { select: self }
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

impl Ordered {
    pub fn build(self) -> Result<Vec<Vec<SQLDataTypes>>, Error> { 
        self.select.build()
    }
    
    pub fn build_single_thread(self) -> Result<Vec<Vec<SQLDataTypes>>, Error> {
        self.select.build_single_thread()
    }
}