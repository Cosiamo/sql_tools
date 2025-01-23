use crate::{data_types::ToSQLData, Error, where_clause::{utils::where_clause_value_format, WhereUpdate}, SQLVariation};

use super::{oracle_sql::{batch_update_oracle, oracle_build_update}, sqlite::sqlite_build_update, SetMatch, UpdateBuilder, UpdateProps, UpdateSet};

impl UpdateProps {
    pub fn set<T: ToSQLData>(self, column: &str, new_value: T) -> UpdateSet {
        let set = vec![
            SetMatch {
                column: column.to_string(),
                value: new_value.fmt_data(),
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
        SQLVariation::SQLite(sqlite_connect) => todo!(),
    }
}