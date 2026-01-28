use std::{
    sync::Arc,
    thread::{self, JoinHandle},
};

use crate::{
    Error, SQLImplementation, data_types::{SQLDataTypes, ToSQLData}, statements::select::SelectProps
};

pub(crate) fn multithread_execution(
    sql_implementation: SQLImplementation,
    handle_execution: fn(
        select_props: Arc<SelectProps>,
        column_size: usize,
        sql: String,
    ) -> Result<Vec<Vec<Box<SQLDataTypes>>>, Error>,
    select_props: SelectProps,
    header: &Vec<Box<SQLDataTypes>>,
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

    let column_size = &header.len() + 0;
    let mut limit = 0 as usize;

    for nthread in 0..nthreads {
        let start: usize = prev + 1;
        let mut end = (nthread + 1) * num.ceil() as usize;
        if end > data_length {
            end = data_length
        }
        if nthread == 0 {
            limit = end
        }

        let offset = start - 1;
        let sql = match sql_implementation {
            SQLImplementation::Oracle(_) => format!("SELECT * FROM ({query}) OFFSET {offset} ROWS FETCH NEXT {limit} ROWS ONLY"),
            SQLImplementation::SQLite(_) => format!("SELECT * FROM ({query}) LIMIT {limit} OFFSET {offset}"),
        };
        // let sql = format!("SELECT * FROM ({query}) WHERE row_num >= {start} and row_num <= {end}");

        let select_props = Arc::clone(&select_props);
        handles.push(thread::spawn(move || {
            handle_execution(select_props, column_size, sql)
        }));

        prev = end;
    }

    let mut res = Vec::new();
    for handle in handles {
        let mut handle = handle.join().unwrap()?;
        res.append(&mut handle);
    }

    if select_props.return_header {
        let header = header
            .iter()
            .map(|head| {
                let head = head.to_string();
                let head = head.split(" as ").collect::<Vec<&str>>();
                let head = head[head.len() - 1];
                let head = head.split(" ").collect::<Vec<&str>>();
                let head = head[head.len() - 1];
                let head = head.split(".").collect::<Vec<&str>>();
                let head = head[head.len() - 1];
                Box::new(head.to_sql_fmt())
            })
            .collect::<Vec<Box<SQLDataTypes>>>();
        let header = vec![header.to_owned()];
        res.splice(..0, header.iter().cloned());
    }

    Ok(res)
}
