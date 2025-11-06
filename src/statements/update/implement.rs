use crate::{
    Error, SQLImplementation, data_types::ToSQLData, statements::update::sql_implementations::{
            oracle::{batch_update_oracle, oracle_build_update},
            sqlite::{batch_update_sqlite, sqlite_build_update},
        }
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
    
    fn build(self) -> Result<(), Error> {
        match self.connect {
            SQLImplementation::Oracle(_) => {
                oracle_build_update(self)?;
                Ok(())
            }
            SQLImplementation::SQLite(_) => {
                sqlite_build_update(self)?;
                Ok(())
            }
        }
    }

    fn build_return_count(self) -> Result<usize, Error> {
        match self.connect {
            SQLImplementation::Oracle(_) => oracle_build_update(self),
            SQLImplementation::SQLite(_) => sqlite_build_update(self),
        }
    }
}

pub fn batch_update(updates: Vec<UpdateProps>) -> Result<(), Error> {
    let connect = &updates[0].connect;
    match connect {
        SQLImplementation::Oracle(_) => batch_update_oracle(updates),
        SQLImplementation::SQLite(_) => batch_update_sqlite(updates),
    }
}
