use std::{
    sync::Arc,
    thread::{self, JoinHandle},
};

use indicatif::ProgressBar;
use iter_grid::{divide_grid, iter_grid};
use sql_fmt::insert_stmt;
use validation::{does_table_exist, get_col_indexes};

use crate::{
    Error, QueryBuilder, SQLVariation,
    data_types::SQLDataTypes,
    sql_implementations::utils::get_dt_indices,
    statements::{
        create::{CreateColumns, CreateDataTypes, ModifyCreateTable},
        insert::InsertProps,
    },
};

pub mod iter_grid;
pub mod sql_fmt;
pub mod validation;

pub(crate) fn oracle_build_insert(
    mut insert_props: InsertProps,
    use_pb: bool,
) -> Result<(), Error> {
    let conn_info = match insert_props.connect {
        SQLVariation::Oracle(oracle_connect) => oracle_connect,
        SQLVariation::SQLite(_) => return Err(Error::SQLVariationError),
    };
    let username_conn = conn_info.username.to_owned();
    let password_conn = conn_info.password.to_owned();
    let connection_string_conn = conn_info.connection_string.to_owned();

    let table_exist = does_table_exist(&insert_props.table, &conn_info)?;
    if !table_exist && insert_props.create {
        let col_type_indexes = get_col_indexes(&insert_props.grid)?;
        let columns = &insert_props
            .header
            .iter()
            .enumerate()
            .map(|(idx, cell)| {
                if col_type_indexes.is_date.contains(&idx) {
                    CreateColumns {
                        name: cell.to_string(),
                        data_type: CreateDataTypes::DATE,
                    }
                } else if col_type_indexes.is_int.contains(&idx) {
                    CreateColumns {
                        name: cell.to_string(),
                        data_type: CreateDataTypes::NUMBER,
                    }
                } else if col_type_indexes.is_float.contains(&idx) {
                    CreateColumns {
                        name: cell.to_string(),
                        data_type: CreateDataTypes::FLOAT,
                    }
                } else {
                    let size = if let Some(val) = col_type_indexes.varchar_size.get(&idx) {
                        val
                    } else {
                        &(1 as usize)
                    };
                    CreateColumns {
                        name: cell.to_string(),
                        data_type: CreateDataTypes::VARCHAR(*size),
                    }
                }
            })
            .collect::<Vec<CreateColumns>>();
        conn_info
            .create()
            .table(&insert_props.table, columns.to_vec())
            .build()?;
    } else if !table_exist && !insert_props.create {
        return Err(Error::TableDoesNotExist);
    }

    let len = &insert_props.grid.len();
    let nthreads = num_cpus::get();
    let num = (len / nthreads + if len % nthreads == 0 { 0 } else { 1 }) as f32;

    let progress_bar = ProgressBar::new(*len as u64);
    let pb = Arc::new(progress_bar);

    let datatype_indices = get_dt_indices(&insert_props.grid);

    if len < &nthreads {
        let query = insert_stmt(
            insert_props.header.len(),
            &insert_props.table,
            &insert_props.header.join(", "),
        );
        let data = insert_props.grid;
        let conn: oracle::Connection =
            oracle::Connection::connect(username_conn, password_conn, connection_string_conn)
                .unwrap();
        let mut batch = conn.batch(&query, data.len()).build()?;
        iter_grid(&mut batch, data, pb, datatype_indices, use_pb)?;
        conn.commit()?;
    } else {
        let mut handles: Vec<JoinHandle<Result<(), Error>>> = Vec::new();
        for n in 0..nthreads {
            let data: Vec<Vec<SQLDataTypes>>;
            if n + 1 < nthreads {
                data = divide_grid(&mut insert_props.grid, num);
            } else {
                data = insert_props.grid.to_owned();
            }
            let query = insert_stmt(
                insert_props.header.len(),
                &insert_props.table,
                &insert_props.header.join(", "),
            );
            let username = username_conn.clone();
            let password = password_conn.clone();
            let connection_string = connection_string_conn.clone();
            let datatype_indices = datatype_indices.clone();
            let pb = Arc::clone(&pb);
            handles.push(thread::spawn(move || {
                // println!("THREAD:{n} DATA:{:?}", data);
                let conn: oracle::Connection =
                    oracle::Connection::connect(username, password, connection_string).unwrap();
                let mut batch = conn.batch(&query, data.len()).build()?;
                iter_grid(&mut batch, data, pb, datatype_indices, use_pb)?;
                conn.commit()?;
                Ok(())
            }))
        }

        for handle in handles {
            handle.join().unwrap()?;
        }
    }

    Ok(())
}
