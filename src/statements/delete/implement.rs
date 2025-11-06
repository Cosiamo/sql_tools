use crate::{
    Error, SQLImplementation, statements::delete::sql_implementations::{oracle::oracle_build_delete, sqlite::sqlite_delete}
};

use super::{DeleteBuilder, DeleteProps};

impl DeleteBuilder for DeleteProps {
    fn build(self) -> Result<(), Error> {
        match self.connect {
            SQLImplementation::Oracle(_) => oracle_build_delete(self),
            SQLImplementation::SQLite(_) => sqlite_delete(self),
        }
    }
}
