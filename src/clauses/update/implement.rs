use crate::{clauses::where_clause::{utils::where_clause_value_format, WhereUpdate}, data_types::ToSQLData, variations::{oracle::update::{batch_update_oracle, oracle_build_update}, sqlite::update::{batch_update_sqlite, sqlite_build_update}}, Error, SQLVariation};

use super::{SetMatch, UpdateBuilder, UpdateProps, UpdateSet};




impl UpdateProps {
    pub fn set<T: ToSQLData>(self, column: &str, new_value: T) -> UpdateSet {
        let set = vec![
            SetMatch {
                column: column.to_string(),
                value: new_value.fmt_data(),
                query: false,
            }
        ];
        UpdateSet {
            connect: self.connect,
            set_match: set,
            table: self.table,
            clause: None,
        }
    }

    pub fn set_query(self, column: &str, query: &str) -> UpdateSet {
        let set = vec![
            SetMatch {
                column: column.to_string(),
                value: query.fmt_data(),
                query: true,
            }
        ];
        UpdateSet {
            connect: self.connect,
            set_match: set,
            table: self.table,
            clause: None,
        }
    }
}

impl UpdateBuilder for UpdateSet {
    fn set<T: ToSQLData>(mut self, column: &str, new_value: T) -> Self {
        self.set_match.push(
            SetMatch {
                column: column.to_string(),
                value: new_value.fmt_data(),
                query: false,
            }
        );
        self
    }

    fn set_query(mut self, column: &str, query: &str) -> Self {
        self.set_match.push(
            SetMatch {
                column: column.to_string(),
                value: query.fmt_data(),
                query: true,
            }
        );
        self
    }

    fn where_in<T: ToSQLData>(self, column: &str, values: Vec<T>) -> WhereUpdate {
        let value = where_clause_value_format(values);
        let where_clause = format!("{} IN ({})", column, value);
        WhereUpdate { 
            query_type: self,
            clause: where_clause
        }
    }

    fn where_not<T: ToSQLData>(self, column: &str, values: Vec<T>) -> WhereUpdate {
        let value = where_clause_value_format(values);
        let where_clause = format!("{} NOT IN ({})", column, value);
        WhereUpdate { 
            query_type: self,
            clause: where_clause
        }
    }

    fn where_null(self, column: &str) -> WhereUpdate {
        let where_clause = format!("{column} IS NULL");
        WhereUpdate { 
            query_type: self,
            clause: where_clause
        }
    }
    
    fn where_not_null(self, column: &str) -> WhereUpdate {
        let where_clause = format!("{column} IS NOT NULL");
        WhereUpdate { 
            query_type: self,
            clause: where_clause
        }
    }

    fn build(self) -> Result<(), Error> {
        match self.connect {
            SQLVariation::Oracle(_) => {
                oracle_build_update(self)?;
                Ok(())
            },
            SQLVariation::SQLite(_) => {
                sqlite_build_update(self)?;
                Ok(())
            },
        }
    }

    fn build_return_count(self) -> Result<usize, Error> {
        match self.connect {
            SQLVariation::Oracle(_) => oracle_build_update(self),
            SQLVariation::SQLite(_) => sqlite_build_update(self),
        }
    }
}

pub fn batch_update(updates: Vec<WhereUpdate>) -> Result<(), Error> {
    let connect = &updates[0].query_type.connect;
    match connect {
        SQLVariation::Oracle(_) => batch_update_oracle(updates),
        SQLVariation::SQLite(_) => batch_update_sqlite(updates),
    }
}