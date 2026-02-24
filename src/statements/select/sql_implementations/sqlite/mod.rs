use crate::{
    Error, SQLImplementation,
    data_types::{SQLDataTypes, ToSQLData},
    statements::select::{
        Column, SelectProps,
        sql_implementations::{
            extract_column_name, multithread::multithread_execution, mutate_query::limit_offset,
            shared_select_operations, sqlite::execution::sqlite_handle_execution,
        },
    },
};

pub mod execution;

fn sqlite_select_setup(
    select_props: &SelectProps,
) -> Result<(Vec<String>, String), Error> {
    let conn_info = select_props.connect.as_sqlite()?;
    let table = &select_props.table;
    let cols = select_props
        .columns
        .iter()
        .map(|col| -> Result<String, Error> {
            let col = match col {
                Column::Name(name) => format!("{}.{}", name.table, name.name),
                Column::Function(function) => format!("{}", function),
                Column::Varchar(varchar) => format!("'{}'", varchar),
                Column::ALL(all) => {
                    let columns = conn_info.table_info(table)?;
                    columns
                        .iter()
                        .map(|col| format!("{}.{}", all, col))
                        .collect::<Vec<String>>()
                        .join(", ")
                }
            };
            Ok(col)
        })
        .collect::<Result<Vec<String>, Error>>()?;
    let columns = cols.join(", ");
    let mut query = format!("SELECT {} FROM {}", columns, table);
    query = shared_select_operations(select_props, query)?;
    query = limit_offset(select_props, query);
    Ok((cols, query))
}

pub(crate) fn build_select_sqlite(
    select_props: SelectProps,
) -> Result<Vec<Vec<Box<SQLDataTypes>>>, Error> {
    let (cols, query) = sqlite_select_setup(&select_props)?;

    let conn_info = select_props.connect.as_sqlite()?;
    let conn = conn_info.initialize_connection()?;

    let columns = cols.join(", ");
    let head = columns.split(",").collect::<Vec<&str>>();
    let header = head
        .iter()
        .map(|col| {
            let col = extract_column_name(col);
            Box::new(col.to_sql_fmt())
        })
        .collect::<Vec<Box<SQLDataTypes>>>();

    let mut count_sql = format!("SELECT COUNT(*) FROM {}", &select_props.table);
    count_sql = shared_select_operations(&select_props, count_sql)?;
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

    multithread_execution(
        SQLImplementation::SQLite(conn_info.to_owned()),
        sqlite_handle_execution,
        select_props,
        &header,
        query,
        count,
    )
}

pub(crate) fn build_select_sqlite_single_thread(
    select_props: SelectProps,
) -> Result<Vec<Vec<Box<SQLDataTypes>>>, Error> {
    let (cols, query) = sqlite_select_setup(&select_props)?;

    let conn_info = select_props.connect.as_sqlite()?;
    let conn = conn_info.initialize_connection()?;

    let columns = cols.join(", ");
    let mut stmt = conn.prepare(&query)?;
    let mut rows = stmt.query([])?;
    let mut res = Vec::new();
    let header = columns.split(",").collect::<Vec<&str>>();
    while let Some(row) = rows.next()? {
        let p = header
            .iter()
            .enumerate()
            .map(|(idx, _)| Box::new(row.get::<usize, SQLDataTypes>(idx).unwrap()))
            .collect::<Vec<Box<SQLDataTypes>>>();
        res.push(p)
    }

    if select_props.return_header {
        let header = header
            .iter()
            .map(|col| Box::new(col.to_sql_fmt()))
            .collect::<Vec<Box<SQLDataTypes>>>();
        let header = header
            .iter()
            .map(|head| {
                let head = head.to_string();
                let head = extract_column_name(&head);
                Box::new(head.to_sql_fmt())
            })
            .collect::<Vec<Box<SQLDataTypes>>>();
        let header = vec![header.to_owned()];
        res.splice(..0, header.iter().cloned());
    }

    Ok(res)
}
