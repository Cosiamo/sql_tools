use std::thread::{self, JoinHandle};

use utils::stmt_res;

use crate::{errors::Error, SQLVariation};

use super::SelectProps;

pub mod utils;

pub fn oracle_build_select(select_props: SelectProps) -> Result<Vec<Vec<Option<String>>>, Error> {
    let conn_info = match select_props.connect {
        SQLVariation::Oracle(connect) => connect,
        // _ => return Err(Error::WrongConnectionType),
    };

    let query: String;

    let count_sql: String;
    match select_props.clause {
        Some(filters) => {
            count_sql = format!("SELECT COUNT(*) FROM {} WHERE {}", &select_props.table, &filters);
            query = format!("SELECT row_number() over (order by rowid) as rn, {} FROM {} WHERE {}", &select_props.columns.join(", "), &select_props.table, filters);
        },
        None => {
            count_sql = format!("SELECT COUNT(*) FROM {}", &select_props.table);
            query = format!("SELECT row_number() over (order by rowid) as rn, {} FROM {}", &select_props.columns.join(", "), &select_props.table);
        },
    }

    let mut count: Option<usize> = None;
    let conn: oracle::Connection = oracle::Connection::connect(&conn_info.username, &conn_info.password, &conn_info.connection_string).unwrap(); 
    let count_query = conn.query(&count_sql, &[])?;
    for res in count_query {
        let row = res?;
        count = Some(row.get_as::<usize>()?);
    };

    let len: usize = if let Some(val) = count { val } else { return Err(Error::CountError) };
    let nthreads = num_cpus::get();
    let num = (len / nthreads + if len % nthreads == 0 { 0 } else { 1 }) as f32;

    let mut handles: Vec<JoinHandle<Result<Vec<Vec<Option<String>>>, Error>>> = Vec::new();

    let mut c: usize = 0;
    let mut prev: usize = 0;

    let col_len = select_props.columns.len() + 1;

    for n in 0..nthreads {
        let start: usize;
        if n == 0 { start = 1 }
        else { start = prev + 1 }
        let mut end = (c + 1) * num.ceil() as usize;
        if end > len { end = len }
        // println!("Start:{}  End:{}", start, end);
        let stmt = format!("SELECT * FROM ({}) WHERE rn >= {} and rn <= {}", query, start, end);
        // println!("{:?}", stmt);
        let username = conn_info.username.to_owned();
        let password = conn_info.password.to_owned();
        let connection_string = conn_info.connection_string.to_owned();
        
        handles.push(thread::spawn(move || {
            let conn: oracle::Connection = oracle::Connection::connect(username, password, connection_string).unwrap(); 
            let stmt = conn.statement(&stmt).build()?;
            stmt_res(stmt, col_len)
        }));
        prev = end;
        c += 1;
    }

    let mut group = Vec::new();
    for handle in handles {
        let mut handle = handle.join().unwrap()?;
        let res = handle.iter_mut().map(|row|{
            let _ = row.remove(0);
            row.to_owned()
        }).collect::<Vec<Vec<Option<String>>>>();
        group.push(res);
    }
    let res = group.concat();
    // res.iter().for_each(|c|{ println!("{:?}", c) });

    Ok(res)
}

pub fn oracle_build_single_thread_select(select_props: SelectProps) -> Result<Vec<Vec<Option<String>>>, Error> {
    let conn_info = match select_props.connect {
        SQLVariation::Oracle(oracle_connect) => oracle_connect,
        // _ => return Err(Error::WrongConnectionType),
    };
    let query = match select_props.clause {
        Some(filters) => format!("SELECT {} FROM {} WHERE {}", &select_props.columns.join(", "), &select_props.table, filters),
        None => format!("SELECT {} FROM {}", &select_props.columns.join(", "), &select_props.table),
    };
    let conn: oracle::Connection = oracle::Connection::connect(conn_info.username, conn_info.password, conn_info.connection_string).unwrap(); 
    let stmt = conn.statement(&query).build()?;
    stmt_res(stmt, select_props.columns.len())
}