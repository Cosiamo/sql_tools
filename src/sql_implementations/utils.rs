use std::collections::{HashMap, HashSet};

use crate::{data_types::SQLDataTypes, statements::insert::DatatypeIndices};

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
        varchar_size: HashMap::new(),
    }
    .find_uniques()
}

fn remove_conflicts(winners: &HashSet<usize>, losers: &mut HashSet<usize>) {
    losers.retain(|v| !winners.contains(v));
}

impl DatatypeIndices {
    pub(crate) fn find_uniques(self) -> Self {
        let varchar_size = self.varchar_size;
        let is_varchar: HashSet<usize> = self.is_varchar.into_iter().collect();
        let mut is_float: HashSet<usize> = self.is_float.into_iter().collect();
        let mut is_int: HashSet<usize> = self.is_int.into_iter().collect();
        let mut is_date: HashSet<usize> = self.is_date.into_iter().collect();

        remove_conflicts(&is_varchar, &mut is_float);
        remove_conflicts(&is_varchar, &mut is_int);
        remove_conflicts(&is_varchar, &mut is_date);
        remove_conflicts(&is_float, &mut is_int);
        remove_conflicts(&is_float, &mut is_date);
        remove_conflicts(&is_int, &mut is_date);

        Self {
            is_varchar: is_varchar.into_iter().collect(),
            is_float: is_float.into_iter().collect(),
            is_int: is_int.into_iter().collect(),
            is_date: is_date.into_iter().collect(),
            varchar_size,
        }
    }
}
