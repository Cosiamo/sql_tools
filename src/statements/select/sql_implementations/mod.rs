use std::{
    sync::Arc,
    thread::{self, JoinHandle},
};

use crate::{
    Error,
    data_types::SQLDataTypes,
    statements::select::{
        SelectProps,
        sql_implementations::mutate_query::{filters, group_by, join_operations, order_by},
    },
};

pub(crate) mod mutate_query;
pub mod oracle;
pub mod sqlite;

pub(crate) fn shared_select_operations(
    select_props: &SelectProps,
    mut query: String,
) -> Result<String, Error> {
    // ===== Joins =====
    if &select_props.joins.len() > &0 {
        query = join_operations(&select_props, query);
    }

    // ===== If filters =====
    query = filters(&select_props, &query);

    // ===== Group By =====
    query = group_by(&select_props, &query);

    // ===== Order By =====
    query = order_by(&select_props, &query)?;

    Ok(query)
}

pub(crate) fn multithread_execution(
    handle_execution: fn(
        select_props: Arc<SelectProps>,
        sql: String,
    ) -> Result<Vec<Vec<Box<SQLDataTypes>>>, Error>,
    select_props: SelectProps,
    query: String,
    count: Option<usize>,
) -> Result<Vec<Vec<Box<SQLDataTypes>>>, Error> {
    // ===== Get length of data then divide by number of threads =====
    let data_length: usize = if let Some(val) = count {
        val
    } else {
        return Err(Error::CountError);
    };
    let nthreads = num_cpus::get();
    let num = (data_length / nthreads + if data_length % nthreads == 0 { 0 } else { 1 }) as f32;

    // ===== Initializing a Vec<JoinHandle> & Arc<SelectProps> for thread safety =====
    let mut handles: Vec<JoinHandle<Result<Vec<Vec<Box<SQLDataTypes>>>, Error>>> = Vec::new();
    let select_props = Arc::new(select_props);

    let mut iteration: usize = 0; // what thread number the loop is on
    let mut prev: usize = 0; // The previous thread's "end" number

    for nthread in 0..nthreads {
        let start: usize;
        if nthread == 0 {
            start = 1
        } else {
            start = prev + 1
        }
        let mut end = (iteration + 1) * num.ceil() as usize;
        if end > data_length {
            end = data_length
        }

        let sql = format!("SELECT * FROM ({query}) WHERE row_num >= {start} and row_num <= {end}");
        // println!("{sql}");
        let select_props = Arc::clone(&select_props);

        handles.push(thread::spawn(move || handle_execution(select_props, sql)));

        prev = end;
        iteration += 1;
    }

    let mut group = Vec::new();
    for handle in handles {
        let mut handle = handle.join().unwrap()?;
        let res = handle
            .iter_mut()
            .map(|row| {
                let _ = row.remove(0); // removing "row_num" from the results
                row.to_owned()
            })
            .collect::<Vec<Vec<Box<SQLDataTypes>>>>();
        group.push(res);
    }
    let mut res = group.concat();
    // res.iter().for_each(|c|{ println!("{:?}", c) });
    if select_props.return_header {
        let header = vec![select_props.columns.iter().map(|column| {
            let column = column.split(".").collect::<Vec<&str>>();
            Box::new(SQLDataTypes::Varchar(column[column.len()-1].to_string()))
        }).collect::<Vec<Box<SQLDataTypes>>>()];
        res.splice(..0, header.iter().cloned());
    }

    Ok(res)
}
