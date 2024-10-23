use oracle::Statement;

use crate::{data_types::SQLDataTypes, errors::Error};

pub fn stmt_res(mut stmt: Statement, column_size: usize) -> Result<Vec<Vec<SQLDataTypes>>, Error> {
    let query = stmt.query(&[])?;
    let mut outer_vec = Vec::new();
    for v in query {
        let p = v?;
        let mut inner_vec = Vec::new();
        for colindx in 0..column_size {
            let a = p.get::<usize, SQLDataTypes>(colindx)?;
            inner_vec.push(a)
        }
        outer_vec.push(inner_vec)
    }

    Ok(outer_vec)
}