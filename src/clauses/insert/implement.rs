use csv::ByteRecord;
use csv_perusal::{utils::assign_bytes, CSVType};

use crate::{data_types::SQLDataTypes, variations::{oracle::insert::oracle_build_insert, sqlite::insert::{sqlite_build_insert, sqlite_build_insert_pb}}, Error, SQLVariation};

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
    fn format_grid_strings(self) -> Result<InsertPropsFormatted, Error> {
        let grid = self.grid.iter().map(|row|{
            row.iter().map(|cell|{
                if let SQLDataTypes::Varchar(t) = cell {
                    // using my csv parser for right now, will rewrite in the future
                    let t = ByteRecord::from(vec![t]);
                    let csv = assign_bytes(vec![t]).unwrap();
                    let mat = match &csv[0][0] {
                        CSVType::Int(val) => Ok(SQLDataTypes::Number(*val).clone()),
                        CSVType::Float(val) => Ok(SQLDataTypes::Float(*val).clone()),
                        CSVType::String(val) => Ok(SQLDataTypes::Varchar(val.clone()).clone()),
                        CSVType::Date(naive_date) => Ok(SQLDataTypes::Date((*naive_date).into()).clone()),
                        CSVType::Time(naive_time) => Ok(SQLDataTypes::Varchar(naive_time.to_string()).clone()),
                        CSVType::DateTime(naive_date_time) => Ok(SQLDataTypes::Date(*naive_date_time).clone()),
                        CSVType::Error(cell_error) => Err(cell_error),
                        CSVType::Empty => Ok(SQLDataTypes::NULL.clone()),
                    };
                    mat.unwrap()
                } else { cell.clone() }
            }).collect::<Vec<SQLDataTypes>>()
        }).collect::<Vec<Vec<SQLDataTypes>>>();

        Ok(
            InsertPropsFormatted {
                insert_props: InsertProps {
                    connect: self.connect,
                    header: self.header,
                    grid,
                    table: self.table,
                }
            }
        )
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
}