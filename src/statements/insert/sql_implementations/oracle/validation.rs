use std::collections::HashMap;

use crate::{
    Error, data_types::SQLDataTypes, sql_implementations::OracleConnect,
    statements::insert::DatatypeIndices,
};

pub(crate) fn does_table_exist(table: &String, conn_info: &OracleConnect) -> Result<bool, Error> {
    let conn = conn_info.initialize_connection()?;
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

pub(crate) fn get_col_indexes(grid: &Vec<Vec<SQLDataTypes>>) -> Result<DatatypeIndices, Error> {
    // get's the 'dominate' datatype from each column
    // weighted in order: VARCHAR2, FLOAT, INT, DATE
    let mut is_varchar: Vec<usize> = Vec::new();
    let mut is_float: Vec<usize> = Vec::new();
    let mut is_int: Vec<usize> = Vec::new();
    let mut is_date: Vec<usize> = Vec::new();

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

    let data_type_indexes = DatatypeIndices {
        is_varchar,
        is_float,
        is_int,
        is_date,
        varchar_size: HashMap::new(),
    };

    Ok(data_type_indexes.find_uniques().get_varchar_sizes(grid))
}

impl DatatypeIndices {
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
                    } else {
                        varchar_size.insert(x_idx, val.len());
                    }
                } else {
                    continue;
                }
            }
        }

        self.varchar_size = varchar_size;
        self
    }
}
