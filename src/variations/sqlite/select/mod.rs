use std::thread::{self, JoinHandle};

use rusqlite::Connection;

use crate::{data_types::SQLDataTypes, statements::select::{JoinType, OrderBy}, Error, SQLVariation};

use super::SelectProps;

pub(crate) fn build_select_sqlite_single_thread(
    mut select_props: SelectProps,
) -> Result<Vec<Vec<Box<SQLDataTypes>>>, Error> {
    let conn_info = match &select_props.connect {
        SQLVariation::Oracle(_) => return Err(Error::SQLVariationError),
        SQLVariation::SQLite(connect) => connect,
    };

    if select_props.columns == vec!["*".to_string()] {
        select_props.columns = conn_info.table_info(&select_props.table.name)?;
    }

    let conn = Connection::open(&conn_info.path.clone())?;

    // ===== Initialize Query =====
    let mut query = format!(
        "SELECT {} FROM {}",
        &select_props.columns.join(", "),
        &select_props.table.query_fmt()
    );
    
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

    // ===== Limit Offset =====
    query = limit_offset(&select_props, query);

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
        select_props.columns = conn_info.table_info(&select_props.table.name)?;
    }

    let conn = Connection::open(&conn_info.path.clone())?;

    // ===== Initialize Queries =====
    let mut query = format!(
        "SELECT row_number() over (order by rowid) as rn, {} FROM {}",
        &select_props.columns.join(", "),
        &select_props.table.query_fmt()
    );
    let mut count_sql= format!("SELECT COUNT(*) FROM {}", &select_props.table.query_fmt());
    
    // ===== Joins =====
    if &select_props.joins.len() > &0 {
        query = join_operations(&select_props, query);
    }
    if &select_props.joins.len() > &0 {
        count_sql = join_operations(&select_props, count_sql);
    }

    // ===== If filters =====
    query = filters(&select_props, &query);
    count_sql = filters(&select_props, &count_sql);

    // ===== Group By =====
    query = group_by(&select_props, &query);
    count_sql = group_by(&select_props, &count_sql);

    // ===== Order By =====
    query = order_by(&select_props, &query)?;
    count_sql = order_by(&select_props, &count_sql)?;

    // ===== Limit Offset =====
    query = limit_offset(&select_props, query);
    count_sql = limit_offset(&select_props, count_sql);

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

pub fn join_operations(select_props: &SelectProps, mut query: String) -> String {
    for join in &select_props.joins {
        let join_type = match join.join_type {
            JoinType::Inner => format!("INNER"),
            JoinType::Outer => format!("OUTER"),
            JoinType::Right => format!("RIGHT"),
            JoinType::Left => format!("LEFT"),
        };
        let join_table = join.table.query_fmt();
        let primary_column = format!("{}.{}", select_props.table.id, join.primary_column);
        let foreign_column = format!("{}.{}", join.table.id, join.foreign_column);
        query = format!("{query} {join_type} JOIN {join_table} ON {primary_column} = {foreign_column}");
    }
    query
}

pub fn filters(select_props: &SelectProps, query: &String) -> String {
    if let Some(filters) =  &select_props.clause {
        format!("{} WHERE {}", query, filters)
    } else {
        query.to_owned()
    }
}

pub fn group_by(select_props: &SelectProps, query: &String) -> String {
    if let Some(group_by) = &select_props.group_by {
        format!("{} GROUP BY {}", query, group_by.join(", "))
    } else {
        query.to_owned()
    }
}

pub fn order_by(select_props: &SelectProps, query: &String) -> Result<String, Error> {
    match &select_props.order_by {
        (None, OrderBy::ASC) => return Err(Error::OrderByError),
        (None, OrderBy::DESC) => return Err(Error::OrderByError),
        (None, OrderBy::None) => Ok(query.to_owned()),
        (Some(column), OrderBy::ASC) => Ok(format!("{} ORDER BY {} ASC", query, column)),
        (Some(column), OrderBy::DESC) => Ok(format!("{} ORDER BY {} DESC", query, column)),
        (Some(_), OrderBy::None) => Ok(query.to_owned()),
    }
}

pub fn limit_offset(select_props: &SelectProps, mut query: String) -> String {
    if let Some(limit) = select_props.limit.limit {
        query = format!("{} LIMIT {}", query, limit);
    }
    if let Some(offset) = select_props.limit.offset {
        query = format!("{} OFFSET {}", query, offset);
    }
    query
}