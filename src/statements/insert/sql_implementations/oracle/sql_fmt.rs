use oracle::Batch;

use crate::Error;

pub(crate) fn bind_cell_to_batch<T: oracle::sql_type::ToSql + std::fmt::Debug>(
    batch: &mut Batch<'_>,
    cell: &T,
    idx: usize,
) -> Result<(), Error> {
    match batch.set(idx + 1, cell) {
        Ok(_) => Ok(()),
        Err(e) => Err(Error::OracleError(e)),
    }
}

pub(crate) fn insert_stmt(length: usize, table: &String, header: &String) -> String {
    let mut values = Vec::new();
    for idx in 0..length {
        values.push(format!(":{}", idx + 1))
    }
    format!(
        "INSERT INTO {} ({}) VALUES ({})",
        table,
        header,
        values.join(", ")
    )
}
