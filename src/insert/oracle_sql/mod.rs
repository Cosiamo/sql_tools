use std::{sync::Arc, thread::{self, JoinHandle}};

use indicatif::ProgressBar;
use iter_grid::{divide_grid, iter_grid, iter_grid_pb};
use sql_fmt::insert_stmt;
use validation::{does_table_exist, get_col_indexes};

use crate::{create::{CreateColumns, CreateDataTypes, ModifyCreateTable}, data_types::SQLDataTypes, Error, QueryBuilder, SQLVariation};

use super::InsertProps;

pub mod validation;
pub mod iter_grid;
pub mod sql_fmt;

pub(crate) fn oracle_build_insert(mut insert_props: InsertProps) -> Result<(), Error> {
    let conn_info = match insert_props.connect {
        SQLVariation::Oracle(oracle_connect) => oracle_connect,
    };
    let username_conn = conn_info.username.to_owned();
    let password_conn = conn_info.password.to_owned();
    let connection_string_conn = conn_info.connection_string.to_owned();
    
    let table_exist = does_table_exist(&insert_props.table, &conn_info)?;
    if !table_exist {
        let col_type_indexes = get_col_indexes(&insert_props.grid)?;
        let columns = &insert_props.header.iter().enumerate().map(|(idx, cell)|{
            if col_type_indexes.is_date.contains(&idx) {
                CreateColumns{ name: cell.to_string(), data_type: CreateDataTypes::DATE }
            } else if col_type_indexes.is_int.contains(&idx) {
                CreateColumns{ name: cell.to_string(), data_type: CreateDataTypes::NUMBER }
            } else if col_type_indexes.is_float.contains(&idx) {
                CreateColumns{ name: cell.to_string(), data_type: CreateDataTypes::FLOAT }
            } else {
                let size = if let Some(val) = col_type_indexes.varchar_size.get(&idx) { val } else { &(1 as usize) };
                CreateColumns{ name: cell.to_string(), data_type: CreateDataTypes::VARCHAR(*size) }
            } 
        }).collect::<Vec<CreateColumns>>();
        conn_info.create().table(&insert_props.table, columns.to_vec()).build()?;
    }

    let len = &insert_props.grid.len();
    let nthreads = num_cpus::get();
    let num = (len / nthreads + if len % nthreads == 0 { 0 } else { 1 }) as f32;

    let mut handles: Vec<JoinHandle<Result<(), Error>>> = Vec::new();
    for n in 0..nthreads {
        let data: Vec<Vec<SQLDataTypes>>;
        if n + 1 < nthreads { data = divide_grid(&mut insert_props.grid, num); } else { data = insert_props.grid.to_owned(); }
        // println!("Thread: {} Data:\n{:?}\n=========", n, data);
        let query = insert_stmt(insert_props.header.len(), &insert_props.table, &insert_props.header.join(", "));
        let username = username_conn.clone();
        let password = password_conn.clone();
        let connection_string = connection_string_conn.clone();
        handles.push(thread::spawn(move || {
            let conn: oracle::Connection = oracle::Connection::connect(username, password, connection_string).unwrap(); 
            let mut batch = conn.batch(&query, data.len()).build()?;
            iter_grid(&mut batch, data)?;
            conn.commit()?;
            Ok(())
        }))
    }

    for handle in handles {
        handle.join().unwrap()?;
    }

    Ok(())
}

pub(crate) fn oracle_build_insert_with_pb(mut insert_props: InsertProps) -> Result<(), Error> {
    let conn_info = match insert_props.connect {
        SQLVariation::Oracle(oracle_connect) => oracle_connect,
    };
    let username_conn = conn_info.username.to_owned();
    let password_conn = conn_info.password.to_owned();
    let connection_string_conn = conn_info.connection_string.to_owned();
    
    let table_exist = does_table_exist(&insert_props.table, &conn_info)?;
    if !table_exist {
        let col_type_indexes = get_col_indexes(&insert_props.grid)?;
        let columns = &insert_props.header.iter().enumerate().map(|(idx, cell)|{
            if col_type_indexes.is_date.contains(&idx) {
                CreateColumns{ name: cell.to_string(), data_type: CreateDataTypes::DATE }
            } else if col_type_indexes.is_int.contains(&idx) {
                CreateColumns{ name: cell.to_string(), data_type: CreateDataTypes::NUMBER }
            } else if col_type_indexes.is_float.contains(&idx) {
                CreateColumns{ name: cell.to_string(), data_type: CreateDataTypes::FLOAT }
            } else {
                let size = if let Some(val) = col_type_indexes.varchar_size.get(&idx) { val } else { &(1 as usize) };
                CreateColumns{ name: cell.to_string(), data_type: CreateDataTypes::VARCHAR(*size) }
            } 
        }).collect::<Vec<CreateColumns>>();
        conn_info.create().table(&insert_props.table, columns.to_vec()).build()?;
    }

    let len = &insert_props.grid.len();
    let progress_bar = ProgressBar::new(*len as u64);
    let nthreads = num_cpus::get();
    let num = (len / nthreads + if len % nthreads == 0 { 0 } else { 1 }) as f32;

    let pb = Arc::new(progress_bar);

    let mut handles: Vec<JoinHandle<Result<(), Error>>> = Vec::new();
    for n in 0..nthreads {
        let data: Vec<Vec<SQLDataTypes>>;
        if n + 1 < nthreads { data = divide_grid(&mut insert_props.grid, num); } else { data = insert_props.grid.to_owned(); }
        // println!("Thread: {} Data:\n{:?}\n=========", n, data);
        let query = insert_stmt(insert_props.header.len(), &insert_props.table, &insert_props.header.join(", "));
        let username = username_conn.clone();
        let password = password_conn.clone();
        let connection_string = connection_string_conn.clone();
        let pb = Arc::clone(&pb);
        handles.push(thread::spawn(move || {
            let conn: oracle::Connection = oracle::Connection::connect(username, password, connection_string).unwrap(); 
            let mut batch = conn.batch(&query, data.len()).build()?;
            iter_grid_pb(&mut batch, data, pb)?;
            conn.commit()?;
            Ok(())
        }))
    }

    for handle in handles {
        handle.join().unwrap()?;
    }

    Ok(())
}