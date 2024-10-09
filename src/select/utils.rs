use oracle::Statement;

use crate::errors::Error;

pub fn stmt_res(mut stmt: Statement, column_size: usize) -> Result<Vec<Vec<Option<String>>>, Error> {
    let query = stmt.query(&[])?;
    let mut outer_vec = Vec::new();
    for v in query {
        let p = v?;
        let mut inner_vec = Vec::new();
        for colindx in 0..column_size {
            let a = p.get::<usize, Option<String>>(colindx)?;
            inner_vec.push(a)
        }
        // println!("{:?}, {:?}", inner_vec, chrono::Utc::now().timestamp_millis());
        outer_vec.push(inner_vec)
    }

    Ok(outer_vec)
}