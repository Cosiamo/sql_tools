use crate::{clauses::where_clause::{utils::where_clause_value_format, WhereSelect}, data_types::{SQLDataTypes, ToSQLData}, variations::{oracle::select::{oracle_build_select, oracle_build_single_thread_select}, sqlite::select::{build_select_sqlite, build_select_sqlite_single_thread}}, Error, SQLVariation};

use super::{group_by::Grouped, Limit, OrderBy, Ordered, SelectBuilder, SelectProps};

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

    fn build(self) -> Result<Vec<Vec<Box<SQLDataTypes>>>, Error> {
        match self.connect {
            SQLVariation::Oracle(_) => oracle_build_select(self),
            SQLVariation::SQLite(_) => build_select_sqlite(self),
        }
    }
    
    fn build_single_thread(self) -> Result<Vec<Vec<Box<SQLDataTypes>>>, Error> {
        match self.connect {
            SQLVariation::Oracle(_) => oracle_build_single_thread_select(self),
            SQLVariation::SQLite(_) => build_select_sqlite_single_thread(self),
        }
    }
    
    fn limit(mut self, limit: usize, offset: Option<usize>) -> Self {
        self.limit = Limit {
            limit: Some(limit),
            offset,
        };
        self
    }
}

impl Ordered {
    pub fn build(self) -> Result<Vec<Vec<Box<SQLDataTypes>>>, Error> { 
        self.select.build()
    }
    
    pub fn build_single_thread(self) -> Result<Vec<Vec<Box<SQLDataTypes>>>, Error> {
        self.select.build_single_thread()
    }
}