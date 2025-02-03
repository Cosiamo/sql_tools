use std::sync::Arc;

use indicatif::ProgressBar;
use oracle::Batch;

use crate::{data_types::SQLDataTypes, Error};

use super::sql_fmt::bind_cell_to_batch;

pub fn iter_grid(mut batch: &mut Batch<'_>, data: Vec<Vec<SQLDataTypes>>) -> Result<(), Error> {
    data.iter().try_for_each(|row| -> Result<(), Error> {
        row.iter().enumerate().try_for_each(|(idx, cell)| -> Result<(), Error> {
            match cell {
                SQLDataTypes::Varchar(val) => bind_cell_to_batch(&mut batch, val, idx),
                SQLDataTypes::Number(val) => bind_cell_to_batch(&mut batch, val, idx),
                SQLDataTypes::Float(val) => bind_cell_to_batch(&mut batch, val, idx),
                SQLDataTypes::Date(val) => bind_cell_to_batch(&mut batch, val, idx),
                SQLDataTypes::NULL => bind_cell_to_batch(&mut batch, &None::<String>, idx),
            }
        })?;
        batch.append_row(&[])?;
        Ok(())
    })?;
    batch.execute()?;
    Ok(())
}

pub fn iter_grid_pb(mut batch: &mut Batch<'_>, data: Vec<Vec<SQLDataTypes>>, progress_bar: Arc<ProgressBar>) -> Result<(), Error> {
    data.iter().try_for_each(|row| -> Result<(), Error> {
        row.iter().enumerate().try_for_each(|(idx, cell)| -> Result<(), Error> {
            match cell {
                SQLDataTypes::Varchar(val) => bind_cell_to_batch(&mut batch, val, idx),
                SQLDataTypes::Number(val) => bind_cell_to_batch(&mut batch, val, idx),
                SQLDataTypes::Float(val) => bind_cell_to_batch(&mut batch, val, idx),
                SQLDataTypes::Date(val) => bind_cell_to_batch(&mut batch, val, idx),
                SQLDataTypes::NULL => bind_cell_to_batch(&mut batch, &None::<String>, idx),
            }
        })?;
        batch.append_row(&[])?;
        progress_bar.inc(1u64);
        Ok(())
    })?;
    batch.execute()?;
    Ok(())
}

pub fn divide_grid(grid: &mut Vec<Vec<SQLDataTypes>>, num: f32) -> Vec<Vec<SQLDataTypes>> {
    grid.splice((0)..(num.ceil() as usize - 1), []).collect()
}