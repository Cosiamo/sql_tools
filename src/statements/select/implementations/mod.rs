use std::{sync::Arc, thread::{self, JoinHandle}};

use crate::{data_types::SQLDataTypes, statements::select::{implementations::mutate_query::{filters, group_by, join_operations, order_by}, SelectProps}, Error};

pub mod oracle;
pub mod sqlite;
pub mod mutate_query;

pub(crate) fn shared_select_operations(
    select_props: &SelectProps, 
    mut query: String
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
    handle_execution: fn (
        select_props: Arc<SelectProps>, 
        stmt: String, 
        col_len: usize
    ) -> Result<Vec<Vec<Box<SQLDataTypes>>>, Error>,
    select_props: SelectProps, 
    query: String, 
    count: Option<usize>,
) -> Result<Vec<Vec<Box<SQLDataTypes>>>, Error> {
    let len: usize = if let Some(val) = count {
        val
    } else {
        return Err(Error::CountError);
    };
    let nthreads = num_cpus::get();
    let num = (len / nthreads + if len % nthreads == 0 { 0 } else { 1 }) as f32;

    let mut handles: Vec<JoinHandle<Result<Vec<Vec<Box<SQLDataTypes>>>, Error>>> = Vec::new();

    let mut c: usize = 0;
    let mut prev: usize = 0;

    let col_len = select_props.columns.len() + 1;
    let select_props = Arc::new(select_props);

    for n in 0..nthreads {
        let start: usize;
        if n == 0 {
            start = 1
        } else {
            start = prev + 1
        }
        let mut end = (c + 1) * num.ceil() as usize;
        if end > len {
            end = len
        }
        // println!("Start:{}  End:{}", start, end);
        let stmt = format!(
            "SELECT * FROM ({}) WHERE rn >= {} and rn <= {}",
            query, start, end
        );
        // println!("{:?}", stmt);
        let select_props = Arc::clone(&select_props);

        handles.push(thread::spawn(move || {
            handle_execution(select_props, stmt, col_len)
        }));
        prev = end;
        c += 1;
    }

    let mut group = Vec::new();
    for handle in handles {
        let mut handle = handle.join().unwrap()?;
        let res = handle
            .iter_mut()
            .map(|row| {
                let _ = row.remove(0);
                row.to_owned()
            })
            .collect::<Vec<Vec<Box<SQLDataTypes>>>>();
        group.push(res);
    }
    let res = group.concat();
    // res.iter().for_each(|c|{ println!("{:?}", c) });

    Ok(res)
}