use std::sync::Arc;

use chrono::NaiveDateTime;
use indicatif::ProgressBar;
use oracle::Batch;

use crate::{Error, data_types::SQLDataTypes, statements::insert::DatatypeIndices};

use super::sql_fmt::bind_cell_to_batch;

pub fn iter_grid(
    mut batch: &mut Batch<'_>,
    data: Vec<Vec<SQLDataTypes>>,
    progress_bar: Arc<ProgressBar>,
    datatype_indices: DatatypeIndices,
    use_pb: bool,
) -> Result<(), Error> {
    data.iter().try_for_each(|row| -> Result<(), Error> {
        row.iter()
            .enumerate()
            .try_for_each(|(idx, cell)| -> Result<(), Error> {
                if let &SQLDataTypes::NULL = cell {
                    if datatype_indices.is_varchar.contains(&idx) {
                        return bind_cell_to_batch(&mut batch, &None::<String>, idx);
                    } else if datatype_indices.is_float.contains(&idx) {
                        return bind_cell_to_batch(&mut batch, &None::<f64>, idx);
                    } else if datatype_indices.is_int.contains(&idx) {
                        return bind_cell_to_batch(&mut batch, &None::<i64>, idx);
                    } else if datatype_indices.is_date.contains(&idx) {
                        return bind_cell_to_batch(&mut batch, &None::<NaiveDateTime>, idx);
                    } else {
                        return bind_cell_to_batch(&mut batch, &None::<String>, idx);
                    }
                } else if datatype_indices.is_varchar.contains(&idx) {
                    let buffer = cell.to_string();
                    let new_val = SQLDataTypes::Varchar(buffer);
                    bind_cell_to_batch(&mut batch, &new_val, idx)
                } else {
                    bind_cell_to_batch(&mut batch, cell, idx)
                }
            })?;
        batch.append_row(&[])?;
        if use_pb {
            progress_bar.inc(1u64);
        }
        Ok(())
    })?;
    batch.execute()?;
    Ok(())
}

pub fn divide_grid(grid: &mut Vec<Vec<SQLDataTypes>>, num: f32) -> Vec<Vec<SQLDataTypes>> {
    let ceil = if num == 1.0 {
        1usize
    } else {
        num.ceil() as usize - 1
    };
    grid.splice((0)..(ceil), []).collect()
}
