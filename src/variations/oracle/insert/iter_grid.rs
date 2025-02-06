use std::sync::Arc;

use indicatif::ProgressBar;
use oracle::Batch;

use crate::{clauses::insert::DatatypeIndices, data_types::SQLDataTypes, Error};

use super::sql_fmt::bind_cell_to_batch;

pub fn iter_grid (
    mut batch: &mut Batch<'_>, 
    data: Vec<Vec<SQLDataTypes>>, 
    progress_bar: Arc<ProgressBar>, 
    _datatype_indices: DatatypeIndices,
    use_pb: bool
) -> Result<(), Error> {
    data.iter().try_for_each(|row| -> Result<(), Error> {
        row.iter().enumerate().try_for_each(|(idx, cell)| -> Result<(), Error> {
            bind_cell_to_batch(&mut batch, cell, idx)
        })?;
        batch.append_row(&[])?;
        if use_pb { progress_bar.inc(1u64); }
        Ok(())
    })?;
    batch.execute()?;
    Ok(())
}

pub fn divide_grid(grid: &mut Vec<Vec<SQLDataTypes>>, num: f32) -> Vec<Vec<SQLDataTypes>> {
    let ceil = if num == 1.0 { 1usize } else { num.ceil() as usize - 1 };
    grid.splice((0)..(ceil), []).collect()
}