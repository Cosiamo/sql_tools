use rusqlite::Connection;

use crate::{
    data_types::SQLDataTypes, statements::select::{
        sql_implementations::{
            multithread::multithread_execution, mutate_query::limit_offset, shared_select_operations, sqlite::execution::sqlite_handle_execution
        }, Column, SelectProps
    }, Error, SQLImplementation
};

pub mod execution;

pub(crate) fn build_select_sqlite(
    mut select_props: SelectProps,
) -> Result<Vec<Vec<Box<SQLDataTypes>>>, Error> {
    let conn_info = match &select_props.connect {
        SQLImplementation::Oracle(_) => return Err(Error::SQLVariationError),
        SQLImplementation::SQLite(connect) => connect,
    };

    let conn = Connection::open(&conn_info.path.clone())?;
    
    let table = &select_props.table;

    if select_props.columns[0].name == "*".to_string() {
        let table_headers = conn_info.table_info(&table)?;
        let mut buffer = vec![];
        for header in table_headers {
            buffer.push(
                Column {
                    name: header,
                    table: table.to_owned(),
                }
            );
        }
        select_props.columns = buffer;
    }

    let cols = &select_props.columns.iter().map(|col|{ format!("{}.{}", col.table, col.name) }).collect::<Vec<String>>();

    let mut query = format!(
        "SELECT row_number() over (order by rowid) as row_num, {} FROM {}",
        &cols.join(", "),
        &table
    );
    let mut count_sql = format!("SELECT COUNT(*) FROM {}", &table);

    query = shared_select_operations(&select_props, query)?;
    count_sql = shared_select_operations(&select_props, count_sql)?;

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

pub(crate) fn build_select_sqlite_single_thread(
    mut select_props: SelectProps,
) -> Result<Vec<Vec<Box<SQLDataTypes>>>, Error> {
    let conn_info = match &select_props.connect {
        SQLImplementation::Oracle(_) => return Err(Error::SQLVariationError),
        SQLImplementation::SQLite(connect) => connect,
    };

    let table = &select_props.table;

    if &select_props.columns[0].name == &"*".to_string() {
        let table_headers = conn_info.table_info(&table)?;
        let mut buffer = vec![];
        for header in table_headers {
            buffer.push(
                Column {
                    name: header,
                    table: table.to_owned(),
                }
            );
        }
        select_props.columns = buffer;
    }

    let conn = Connection::open(&conn_info.path.clone())?;

    let cols = &select_props.columns.iter().map(|col|{ format!("{}.{}", col.table, col.name) }).collect::<Vec<String>>();

    let mut query = format!(
        "SELECT {} FROM {}",
        &cols.join(", "),
        &table,
    );

    query = shared_select_operations(&select_props, query)?;

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

    if select_props.return_header {
        let header = vec![select_props.columns.iter().map(|column| {
            let column = &column.name;
            Box::new(SQLDataTypes::Varchar(column.to_owned()))
        }).collect::<Vec<Box<SQLDataTypes>>>()];
        res.splice(..0, header.iter().cloned());
    }

    Ok(res)
}
