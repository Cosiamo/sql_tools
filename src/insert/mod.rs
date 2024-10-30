use csv::ByteRecord;
use csv_perusal::{utils::assign_bytes, CSVType};
use oracle_sql::{oracle_build_insert, oracle_build_insert_with_pb};

use crate::{data_types::SQLDataTypes, errors::Error, SQLVariation};

pub mod oracle_sql;

#[derive(Debug)]
pub struct InsertProps {
    pub connect: SQLVariation,
    pub header: Vec<String>,
    pub grid: Vec<Vec<SQLDataTypes>>,
    pub table: String,
}

#[derive(Debug)]
pub struct InsertPropsFormatted {
    pub insert_props: InsertProps,
}

impl InsertPropsFormatted {
    pub fn build(self) -> Result<(), Error> {
        self.insert_props.build()
    }

    pub fn build_with_progress_bar(self) -> Result<(), Error> {
        self.insert_props.build_with_progress_bar()
    }
}

pub trait InsertBuilder {
    fn format_grid_strings(self) -> Result<InsertPropsFormatted, Error>;
    fn build(self) -> Result<(), Error>;
    fn build_with_progress_bar(self) -> Result<(), Error>;
}

impl InsertBuilder for InsertProps {
    fn format_grid_strings(self) -> Result<InsertPropsFormatted, Error> {
        let grid = self.grid.iter().map(|row|{
            row.iter().map(|cell|{
                if let SQLDataTypes::VARCHAR(t) = cell {
                    // using my csv parser for right now, will rewrite in the future
                    let t = ByteRecord::from(vec![t]);
                    let csv = assign_bytes(vec![t]).unwrap();
                    let mat = match &csv[0][0] {
                        CSVType::Int(val) => Ok(SQLDataTypes::NUMBER(*val).clone()),
                        CSVType::Float(val) => Ok(SQLDataTypes::FLOAT(*val).clone()),
                        CSVType::String(val) => Ok(SQLDataTypes::VARCHAR(val.clone()).clone()),
                        CSVType::Date(naive_date) => Ok(SQLDataTypes::DATE((*naive_date).into()).clone()),
                        CSVType::Time(naive_time) => Ok(SQLDataTypes::VARCHAR(naive_time.to_string()).clone()),
                        CSVType::DateTime(naive_date_time) => Ok(SQLDataTypes::DATE(*naive_date_time).clone()),
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
            SQLVariation::Oracle(_) => oracle_build_insert(self),
        }
    }

    fn build_with_progress_bar(self) -> Result<(), Error> {
        match self.connect {
            SQLVariation::Oracle(_) => oracle_build_insert_with_pb(self),
        }
    }
}