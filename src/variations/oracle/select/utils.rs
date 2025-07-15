use oracle::Statement;

use crate::{Error, data_types::SQLDataTypes};

pub(crate) fn stmt_res(
    mut stmt: Statement,
    column_size: usize,
) -> Result<Vec<Vec<Box<SQLDataTypes>>>, Error> {
    let query = stmt.query(&[])?;
    let mut outer_vec = Vec::new();
    for v in query {
        let p = v?;
        let mut inner_vec = Vec::new();
        for col_idx in 0..column_size {
            let a = p.get::<usize, SQLDataTypes>(col_idx)?;
            inner_vec.push(Box::new(a))
        }
        outer_vec.push(inner_vec)
    }

    Ok(outer_vec)
}
