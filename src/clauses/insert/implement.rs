use crate::{
    Error, SQLVariation,
    variations::{
        oracle::insert::oracle_build_insert,
        sqlite::insert::{sqlite_build_insert, sqlite_build_insert_pb},
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
        for y_idx in 0..self.grid.len() {
            for x_idx in 0..self.grid[y_idx].len() {
                self.grid[y_idx][x_idx].format_data_types();
            }
        }

        Ok(InsertPropsFormatted {
            insert_props: InsertProps {
                connect: self.connect,
                header: self.header,
                grid: self.grid,
                table: self.table,
                create: self.create,
            },
        })
    }

    fn build(self) -> Result<(), Error> {
        match self.connect {
            SQLVariation::Oracle(_) => oracle_build_insert(self, false),
            SQLVariation::SQLite(_) => sqlite_build_insert(self),
        }
    }

    fn build_with_progress_bar(self) -> Result<(), Error> {
        match self.connect {
            SQLVariation::Oracle(_) => oracle_build_insert(self, true),
            SQLVariation::SQLite(_) => sqlite_build_insert_pb(self),
        }
    }

    fn create_table(mut self) -> Self {
        self.create = true;
        self
    }
}
