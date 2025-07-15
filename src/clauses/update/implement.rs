use crate::{
    Error, SQLVariation,
    clauses::where_clause::utils::where_clause_value_format,
    data_types::ToSQLData,
    variations::{
        oracle::update::{batch_update_oracle, oracle_build_update},
        sqlite::update::{batch_update_sqlite, sqlite_build_update},
    },
};

use super::{SetMatch, UpdateBuilder, UpdateInitialization, UpdateProps};

impl UpdateInitialization {
    pub fn set<T: ToSQLData>(self, column: &str, new_value: T) -> UpdateProps {
        let set = vec![SetMatch {
            column: column.to_string(),
            value: new_value.fmt_data(),
            query: false,
        }];
        UpdateProps {
            connect: self.connect,
            set_match: set,
            table: self.table,
            clause: None,
        }
    }

    pub fn set_query(self, column: &str, query: &str) -> UpdateProps {
        let set = vec![SetMatch {
            column: column.to_string(),
            value: query.fmt_data(),
            query: true,
        }];
        UpdateProps {
            connect: self.connect,
            set_match: set,
            table: self.table,
            clause: None,
        }
    }
}

impl UpdateBuilder for UpdateProps {
    fn set<T: ToSQLData>(mut self, column: &str, new_value: T) -> Self {
        self.set_match.push(SetMatch {
            column: column.to_string(),
            value: new_value.fmt_data(),
            query: false,
        });
        self
    }

    fn set_query(mut self, column: &str, query: &str) -> Self {
        self.set_match.push(SetMatch {
            column: column.to_string(),
            value: query.fmt_data(),
            query: true,
        });
        self
    }

    fn where_in<T: ToSQLData>(mut self, column: &str, values: Vec<T>) -> Self {
        let value = where_clause_value_format(values);
        let where_clause = format!("{} IN ({})", column, value);
        self.clause = Some(where_clause);
        self
    }

    fn where_not<T: ToSQLData>(mut self, column: &str, values: Vec<T>) -> Self {
        let value = where_clause_value_format(values);
        let where_clause = format!("{} NOT IN ({})", column, value);
        self.clause = Some(where_clause);
        self
    }

    fn where_null(mut self, column: &str) -> Self {
        let where_clause = format!("{column} IS NULL");
        self.clause = Some(where_clause);
        self
    }

    fn where_not_null(mut self, column: &str) -> Self {
        let where_clause = format!("{column} IS NOT NULL");
        self.clause = Some(where_clause);
        self
    }

    fn build(self) -> Result<(), Error> {
        match self.connect {
            SQLVariation::Oracle(_) => {
                oracle_build_update(self)?;
                Ok(())
            }
            SQLVariation::SQLite(_) => {
                sqlite_build_update(self)?;
                Ok(())
            }
        }
    }

    fn build_return_count(self) -> Result<usize, Error> {
        match self.connect {
            SQLVariation::Oracle(_) => oracle_build_update(self),
            SQLVariation::SQLite(_) => sqlite_build_update(self),
        }
    }
}

pub fn batch_update(updates: Vec<UpdateProps>) -> Result<(), Error> {
    let connect = &updates[0].connect;
    match connect {
        SQLVariation::Oracle(_) => batch_update_oracle(updates),
        SQLVariation::SQLite(_) => batch_update_sqlite(updates),
    }
}
