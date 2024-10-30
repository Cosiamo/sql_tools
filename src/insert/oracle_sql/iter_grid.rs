use oracle::Batch;

use crate::{data_types::SQLDataTypes, errors::Error};

use super::sql_fmt::bind_cell_to_batch;

pub fn iter_grid(mut batch: &mut Batch<'_>, data: Vec<Vec<SQLDataTypes>>) -> Result<(), Error> {
    data.iter().try_for_each(|row| -> Result<(), Error> {
        row.iter().enumerate().try_for_each(|(idx, cell)| -> Result<(), Error> {
            match cell {
                SQLDataTypes::VARCHAR(val) => bind_cell_to_batch(&mut batch, val, idx),
                SQLDataTypes::NUMBER(val) => bind_cell_to_batch(&mut batch, val, idx),
                SQLDataTypes::FLOAT(val) => bind_cell_to_batch(&mut batch, val, idx),
                SQLDataTypes::DATE(val) => bind_cell_to_batch(&mut batch, val, idx),
                SQLDataTypes::NULL => bind_cell_to_batch(&mut batch, &None::<String>, idx),
            }
        })?;
        batch.append_row(&[])?;
        Ok(())
    })?;
    batch.execute()?;
    Ok(())
}

pub fn divide_grid(grid: &mut Vec<Vec<SQLDataTypes>>, num: f32) -> Vec<Vec<SQLDataTypes>> {
    grid.splice((0)..(num.ceil() as usize - 1), []).collect()
}