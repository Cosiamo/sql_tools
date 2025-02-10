use itertools::Itertools;

use crate::{clauses::insert::DatatypeIndices, data_types::SQLDataTypes};

pub(crate) fn get_dt_indices(data: &Vec<Vec<SQLDataTypes>>) -> DatatypeIndices {
    let mut is_varchar: Vec<usize> = Vec::new();
    let mut is_float: Vec<usize> = Vec::new();
    let mut is_int: Vec<usize> = Vec::new();
    let mut is_date: Vec<usize> = Vec::new();

    if data.len() == 1 {
        match data[0][0] {
            SQLDataTypes::Varchar(_) => is_varchar.push(0),
            SQLDataTypes::Number(_) => is_int.push(0),
            SQLDataTypes::Float(_) => is_float.push(0),
            SQLDataTypes::Date(_) => is_date.push(0),
            SQLDataTypes::NULL => is_varchar.push(0),
        }
    } else {
        for y_idx in 1..data.len() {
            println!("IDX:{y_idx} DI: {:?}", &data[y_idx]);
            for x_idx in 0..data[y_idx].len() {
                let cell = &data[y_idx][x_idx];
                match cell {
                    SQLDataTypes::Varchar(_) => is_varchar.push(x_idx),
                    SQLDataTypes::Number(_) => is_int.push(x_idx),
                    SQLDataTypes::Float(_) => is_float.push(x_idx),
                    SQLDataTypes::Date(_) => is_date.push(x_idx),
                    SQLDataTypes::NULL => continue,
                }
            }
        }
    }

    DatatypeIndices {
        is_varchar,
        is_float,
        is_int,
        is_date,
    }.find_uniques()
}

impl DatatypeIndices {
    pub(crate) fn find_uniques(mut self) -> Self {
        let is_varchar = self.is_varchar.into_iter().unique().collect::<Vec<usize>>();
        for x_idx in is_varchar.iter() {
            if self.is_float.contains(x_idx) { self.is_float.retain(|v| *v != *x_idx); }
            else if self.is_int.contains(x_idx) { self.is_int.retain(|v| *v != *x_idx); }
            else if self.is_date.contains(x_idx) { self.is_date.retain(|v| *v != *x_idx); }
            else { continue }
        };
        let is_float = self.is_float.into_iter().unique().collect::<Vec<usize>>();
        for x_idx in is_float.iter() {
            if self.is_int.contains(x_idx) { self.is_int.retain(|v| *v != *x_idx); }
            else if self.is_date.contains(x_idx) { self.is_date.retain(|v| *v != *x_idx); }
            else { continue }
        }
        let is_int = self.is_int.into_iter().unique().collect::<Vec<usize>>();
        for x_idx in is_int.iter() {
            if self.is_date.contains(x_idx) { self.is_date.retain(|v| *v != *x_idx); }
            else { continue }
        }
        let is_date = self.is_date.into_iter().unique().collect::<Vec<usize>>();
        Self {
            is_varchar,
            is_float,
            is_int,
            is_date,
        }
    }
}