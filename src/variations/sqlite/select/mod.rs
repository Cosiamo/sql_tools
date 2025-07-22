use std::thread::{self, JoinHandle};

use rusqlite::Connection;

use crate::{Error, SQLVariation, statements::select::OrderBy, data_types::SQLDataTypes};

use super::SelectProps;

pub(crate) fn build_select_sqlite_single_thread(
    mut select_props: SelectProps,
) -> Result<Vec<Vec<Box<SQLDataTypes>>>, Error> {
    let conn_info = match &select_props.connect {
        SQLVariation::Oracle(_) => return Err(Error::SQLVariationError),
        SQLVariation::SQLite(connect) => connect,
    };

    if select_props.columns == vec!["*".to_string()] {
        select_props.columns = conn_info.table_info(&select_props.table)?;
    }

    let conn = Connection::open(&conn_info.path.clone())?;

    let mut query = match select_props.clause {
        Some(filters) => format!(
            "SELECT {} FROM {} WHERE {}",
            &select_props.columns.join(", "),
            &select_props.table,
            filters
        ),
        None => format!(
            "SELECT {} FROM {}",
            &select_props.columns.join(", "),
            &select_props.table
        ),
    };

    query = if let Some(group_by) = select_props.group_by {
        format!("{} GROUP BY {}", query, group_by.join(", "))
    } else {
        query
    };

    match select_props.order_by {
        (None, OrderBy::ASC) => return Err(Error::OrderByError),
        (None, OrderBy::DESC) => return Err(Error::OrderByError),
        (None, OrderBy::None) => query = query,
        (Some(column), OrderBy::ASC) => query = format!("{} ORDER BY {} ASC", query, column),
        (Some(column), OrderBy::DESC) => query = format!("{} ORDER BY {} DESC", query, column),
        (Some(_), OrderBy::None) => query = query,
    }

    if let Some(limit) = select_props.limit.limit {
        query = format!("{} LIMIT {}", query, limit);
        if let Some(offset) = select_props.limit.offset {
            query = format!("{} OFFSET {}", query, offset)
        }
    };

    let mut stmt = conn.prepare(&query)?;
    let mut rows = stmt.query([])?;
    let mut res = Vec::new();
    while let Some(row) = rows.next()? {
        let p = select_props
            .columns
            .iter()
            .enumerate()
            .map(|(idx, _)| Box::new(row.get::<usize, SQLDataTypes>(idx).unwrap()))
            .collect::<Vec<Box<SQLDataTypes>>>();
        res.push(p)
    }

    Ok(res)
}

pub(crate) fn build_select_sqlite(
    mut select_props: SelectProps,
) -> Result<Vec<Vec<Box<SQLDataTypes>>>, Error> {
    let conn_info = match &select_props.connect {
        SQLVariation::Oracle(_) => return Err(Error::SQLVariationError),
        SQLVariation::SQLite(connect) => connect,
    };

    if select_props.columns == vec!["*".to_string()] {
        select_props.columns = conn_info.table_info(&select_props.table)?;
    }

    let conn = Connection::open(&conn_info.path.clone())?;

    let mut query: String;

    let count_sql: String;
    match select_props.clause {
        Some(filters) => {
            count_sql = format!(
                "SELECT COUNT(*) FROM {} WHERE {}",
                &select_props.table, &filters
            );
            query = format!(
                "SELECT row_number() over (order by rowid) as rn, {} FROM {} WHERE {}",
                &select_props.columns.join(", "),
                &select_props.table,
                filters
            );
        }
        None => {
            count_sql = format!("SELECT COUNT(*) FROM {}", &select_props.table);
            query = format!(
                "SELECT row_number() over (order by rowid) as rn, {} FROM {}",
                &select_props.columns.join(", "),
                &select_props.table
            );
        }
    }
    query = if let Some(group_by) = select_props.group_by {
        format!("{} GROUP BY {}, rowid", query, group_by.join(", "))
    } else {
        query
    };

    match select_props.order_by {
        (None, OrderBy::ASC) => return Err(Error::OrderByError),
        (None, OrderBy::DESC) => return Err(Error::OrderByError),
        (None, OrderBy::None) => query = query,
        (Some(column), OrderBy::ASC) => query = format!("{} ORDER BY {} ASC", query, column),
        (Some(column), OrderBy::DESC) => query = format!("{} ORDER BY {} DESC", query, column),
        (Some(_), OrderBy::None) => query = query,
    }

    if let Some(limit) = select_props.limit.limit {
        query = format!("{} LIMIT {}", query, limit);
        if let Some(offset) = select_props.limit.offset {
            query = format!("{} OFFSET {}", query, offset)
        }
    };

    let mut count: Option<usize> = None;
    let mut stmt = conn.prepare(&count_sql)?;
    let count_query = stmt.query_map([], |row| Ok(row.get(0)?))?;
    for res in count_query {
        let row = res?;
        count = if let Some(val) = row {
            Some(val)
        } else {
            Some(0usize)
        }
    }

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
        let sql = format!(
            "SELECT * FROM ({}) WHERE rn >= {} and rn <= {}",
            query, start, end
        );
        // println!("{:?}", stmt);
        let path = conn_info.path.to_owned();

        handles.push(thread::spawn(move || {
            let conn = Connection::open(path.clone())?;
            let mut stmt = conn.prepare(&sql)?;
            let mut rows = stmt.query([])?;
            let mut res = Vec::new();
            while let Some(row) = rows.next()? {
                // let p = select_props.columns.iter().enumerate().map(|(idx, _)| {
                //     row.get::<usize, SQLDataTypes>(idx).unwrap()
                // }).collect::<Vec<SQLDataTypes>>();
                let mut p = Vec::new();
                for idx in 0..col_len {
                    p.push(Box::new(row.get::<usize, SQLDataTypes>(idx).unwrap()))
                }
                res.push(p)
            }
            Ok(res)
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
