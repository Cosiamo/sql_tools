use crate::{
    Error, SQLImplementation,
    statements::insert::sql_implementations::{
        oracle::oracle_build_insert,
        sqlite::{sqlite_build_insert, sqlite_build_insert_pb},
    },
};

use super::{InsertBuilder, InsertProps, InsertPropsFormatted};

impl InsertPropsFormatted {
    pub fn build(self) -> Result<(), Error> {
        self.insert_props.build()
    }

    pub fn build_with_progress_bar(self) -> Result<(), Error> {
        self.insert_props.build_with_progress_bar()
    }
}

impl InsertBuilder for InsertProps {
    fn format_grid_strings(mut self) -> Result<InsertPropsFormatted, Error> {
        for row in &mut self.grid {
            for cell in row {
                cell.format_data_types();
            }
        }

        Ok(InsertPropsFormatted { insert_props: self })
    }

    fn build(self) -> Result<(), Error> {
        match self.connect {
            SQLImplementation::Oracle(_) => oracle_build_insert(self, false),
            SQLImplementation::SQLite(_) => sqlite_build_insert(self),
        }
    }

    fn build_with_progress_bar(self) -> Result<(), Error> {
        match self.connect {
            SQLImplementation::Oracle(_) => oracle_build_insert(self, true),
            SQLImplementation::SQLite(_) => sqlite_build_insert_pb(self),
        }
    }

    fn create_table(mut self) -> Self {
        self.create = true;
        self
    }
}
