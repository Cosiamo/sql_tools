use oracle::Batch;

use crate::Error;

pub(crate) fn bind_cell_to_batch<T: oracle::sql_type::ToSql + std::fmt::Debug>(batch: &mut Batch<'_>, cell: &T, idx: usize) -> Result<(), Error> {
    match batch.set(idx + 1, cell) {
        Ok(_) => Ok(()),
        Err(e) => {
            println!("CAUSING ERROR:{:?}", cell);
            Err(Error::OracleError(e))
        },
    }
}

pub(crate) fn insert_stmt(length: usize, table: &String, header: &String) -> String {
    let mut values = Vec::new();
    for idx in 0..length {
        values.push([":", &(idx + 1).to_string()].concat())
    }
    format!("INSERT INTO {} ({}) VALUES ({})", table, header, values.join(", "))
}