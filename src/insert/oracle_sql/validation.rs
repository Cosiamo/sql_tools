use std::collections::HashMap;

use itertools::Itertools;

use crate::{data_types::SQLDataTypes, errors::Error, sql_variations::OracleConnect};

pub fn does_table_exist(table: &String, conn_info: &OracleConnect) -> Result<bool, Error> {
    let conn: oracle::Connection = oracle::Connection::connect(&conn_info.username, &conn_info.password, &conn_info.connection_string).unwrap(); 
    let mut existing_tables = conn
        .statement("SELECT table_name FROM user_tables")
        .build()?;
    for row_result in existing_tables.query_as::<String>(&[])? {
        let name = row_result?;
        match name.eq_ignore_ascii_case(&table) {
            true => return Ok(true),
            false => continue,
        }
    }
    Ok(false)
}

#[derive(Debug)]
pub struct DatatypeIndexes {
    pub is_varchar: Vec<usize>,
    pub is_float: Vec<usize>,
    pub is_int: Vec<usize>,
    pub is_date: Vec<usize>,
    pub varchar_size: HashMap<usize, usize>,
}

#[derive(Debug)]
pub struct VarcharProps {
    pub index: usize,
    pub size: usize,
}

pub fn get_col_indexes(grid: &Vec<Vec<SQLDataTypes>>) -> Result<DatatypeIndexes, Error> {
    // get's the 'dominate' datatype from each column
    // weighted in order: VARCHAR2, FLOAT, INT, DATE
    let mut is_varchar: Vec<usize> = Vec::new();
    let mut is_float: Vec<usize> = Vec::new();
    let mut is_int: Vec<usize> = Vec::new();
    let mut is_date: Vec<usize> = Vec::new();
    let varchar_size = HashMap::new();

    for row in grid.iter() {
        for (x_idx, cell) in row.iter().enumerate() {
            match cell {
                SQLDataTypes::Varchar(_) => is_varchar.push(x_idx),
                SQLDataTypes::Number(_) => is_int.push(x_idx),
                SQLDataTypes::Float(_) => is_float.push(x_idx),
                SQLDataTypes::Date(_) => is_date.push(x_idx),
                SQLDataTypes::NULL => continue,
            }
        }
    }

    let data_type_indexes = DatatypeIndexes {
        is_varchar,
        is_float,
        is_int,
        is_date,
        varchar_size,
    };

    Ok(data_type_indexes.find_uniques().get_varchar_sizes(grid))
}


impl DatatypeIndexes {
    pub(crate) fn find_uniques(mut self) -> Self {
        let is_varchar = self.is_varchar.into_iter().unique().collect::<Vec<usize>>();
        for x_index in is_varchar.iter() {
            if self.is_float.contains(x_index) { self.is_float.retain(|v| *v != *x_index); }
            else if self.is_int.contains(x_index) { self.is_int.retain(|v| *v != *x_index); }
            else if self.is_date.contains(x_index) { self.is_date.retain(|v| *v != *x_index); }
            else { continue }
        };
        let is_float = self.is_float.into_iter().unique().collect::<Vec<usize>>();
        for x_index in is_float.iter() {
            if self.is_int.contains(x_index) { self.is_int.retain(|v| *v != *x_index); }
            else if self.is_date.contains(x_index) { self.is_date.retain(|v| *v != *x_index); }
            else { continue }
        }
        let is_int = self.is_int.into_iter().unique().collect::<Vec<usize>>();
        for x_index in is_int.iter() {
            if self.is_date.contains(x_index) { self.is_date.retain(|v| *v != *x_index); }
            else { continue }
        }
        let is_date = self.is_date.into_iter().unique().collect::<Vec<usize>>();
        Self {
            is_varchar,
            is_float,
            is_int,
            is_date,
            varchar_size: self.varchar_size,
        }
    }

    pub(crate) fn get_varchar_sizes(mut self, grid: &Vec<Vec<SQLDataTypes>>) -> Self {
        let mut varchar_size = HashMap::new();
        for row in grid.iter() {
            for (x_idx, cell) in row.iter().enumerate() {
                if self.is_varchar.contains(&x_idx) {
                    let val = match cell {
                        SQLDataTypes::Varchar(val) => val.to_owned(),
                        SQLDataTypes::Number(val) => format!("{}", val),
                        SQLDataTypes::Float(val) => format!("{}", val),
                        SQLDataTypes::Date(val) => format!("{}", val),
                        SQLDataTypes::NULL => format!(""),
                    };
                    if let Some(existing_size) = varchar_size.get(&x_idx) {
                        if val.len() > *existing_size {
                            varchar_size.remove(&x_idx); 
                            varchar_size.insert(x_idx, val.len()); 
                        }
                    } else { varchar_size.insert(x_idx, val.len()); }
                } else { continue; }
            }
        };

        self.varchar_size = varchar_size;
        self
    }
}