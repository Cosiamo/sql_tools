use std::sync::Arc;

use rusqlite::Connection;

use crate::{data_types::SQLDataTypes, statements::select::{implementations::{multithread_execution, mutate_query::limit_offset, shared_select_operations}, SelectProps}, Error, SQLVariation};

pub(crate) fn build_select_sqlite(
    mut select_props: SelectProps,
) -> Result<Vec<Vec<Box<SQLDataTypes>>>, Error> {
    let conn_info = match &select_props.connect {
        SQLVariation::Oracle(_) => return Err(Error::SQLVariationError),
        SQLVariation::SQLite(connect) => connect,
    };

    let conn = Connection::open(&conn_info.path.clone())?;

    if select_props.columns == vec!["*".to_string()] {
        select_props.columns = conn_info.table_info(&select_props.table.name)?;
    }

    // ===== Initialize Queries =====
    let mut query = format!(
        "SELECT row_number() over (order by rowid) as rn, {} FROM {}",
        &select_props.columns.join(", "),
        &select_props.table.query_fmt()
    );
    let mut count_sql= format!("SELECT COUNT(*) FROM {}", &select_props.table.query_fmt());
    
    query = shared_select_operations(&select_props, query)?;
    count_sql = shared_select_operations(&select_props, count_sql)?;
    
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

    multithread_execution(sqlite_handle_execution, select_props, query, count)
}

pub fn sqlite_handle_execution(
    select_props: Arc<SelectProps>, 
    stmt: String, 
    col_len: usize
) -> Result<Vec<Vec<Box<SQLDataTypes>>>, Error> {
    let path = match &select_props.connect {
        SQLVariation::Oracle(_) => return Err(Error::SQLVariationError),
        SQLVariation::SQLite(connect) => &connect.path,
    };
    let conn = Connection::open(path.clone())?;
    let mut stmt = conn.prepare(&stmt)?;
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
}

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
    
    query = shared_select_operations(&select_props, query)?;

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