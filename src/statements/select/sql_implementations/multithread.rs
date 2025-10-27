use std::{sync::Arc, thread::{self, JoinHandle}};

use crate::{data_types::SQLDataTypes, statements::select::SelectProps, Error};

pub(crate) fn multithread_execution(
    handle_execution: fn(
        select_props: Arc<SelectProps>,
        sql: String,
    ) -> Result<Vec<Vec<Box<SQLDataTypes>>>, Error>,
    select_props: SelectProps,
    query: String,
    count: Option<usize>,
) -> Result<Vec<Vec<Box<SQLDataTypes>>>, Error> {
    let data_length: usize = if let Some(val) = count {
        val
    } else {
        return Err(Error::CountError);
    };
    let nthreads = num_cpus::get();
    let num = (data_length / nthreads + if data_length % nthreads == 0 { 0 } else { 1 }) as f32;

    let mut handles: Vec<JoinHandle<Result<Vec<Vec<Box<SQLDataTypes>>>, Error>>> = Vec::new();
    let select_props = Arc::new(select_props);

    let mut prev: usize = 0; // The previous thread's "end" number

    for nthread in 0..nthreads {
        let start: usize = prev + 1; 
        let mut end = (nthread + 1) * num.ceil() as usize; 
        if end > data_length { end = data_length }

        let sql = format!("SELECT * FROM ({query}) WHERE row_num >= {start} and row_num <= {end}");
        
        let select_props = Arc::clone(&select_props);
        handles.push(thread::spawn(move || handle_execution(select_props, sql)));

        prev = end;
    }

    let mut res = Vec::new();
    for handle in handles {
        let mut handle = handle.join().unwrap()?;
        res.append(&mut handle);
    }

    if select_props.return_header {
        let col_fmt = select_props.columns.iter().map(|column| {
            let column = column.name.split(".").collect::<Vec<&str>>();
            Box::new(SQLDataTypes::Varchar(column[column.len()-1].to_string()))
        }).collect::<Vec<Box<SQLDataTypes>>>();
        let header = vec![col_fmt];
        res.splice(..0, header.iter().cloned());
    }

    Ok(res)
}
