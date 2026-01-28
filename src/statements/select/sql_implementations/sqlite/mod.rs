use crate::{
    Error, SQLImplementation,
    data_types::{SQLDataTypes, ToSQLData},
    statements::select::{
        SelectProps,
        sql_implementations::{
            multithread::multithread_execution, mutate_query::limit_offset,
            shared_select_operations, sqlite::execution::sqlite_handle_execution,
        },
    },
};

pub mod execution;

pub(crate) fn build_select_sqlite(
    select_props: SelectProps,
) -> Result<Vec<Vec<Box<SQLDataTypes>>>, Error> {
    let conn_info = match &select_props.connect {
        SQLImplementation::Oracle(_) => return Err(Error::SQLVariationError),
        SQLImplementation::SQLite(connect) => connect,
    };

    let conn = conn_info.initialize_connection()?;

    let table = &select_props.table;
    let cols = &select_props
        .columns
        .iter()
        .map(|col| -> Result<String, Error> {
            let col = match col {
                crate::statements::select::Column::Name(name) => {
                    format!("{}.{}", name.table, name.name)
                }
                crate::statements::select::Column::Function(function) => format!("{}", function),
                crate::statements::select::Column::Varchar(varchar) => format!("'{}'", varchar),
                crate::statements::select::Column::ALL(all) => {
                    let columns = conn_info.table_info(&table)?;
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

    let columns = &cols.join(", ");
    let head = &columns.split(",").collect::<Vec<&str>>();
    let header = head
        .iter()
        .map(|col| {
            let col = col.split(".").collect::<Vec<&str>>();
            let col = col[col.len() - 1];
            let col = col.split(" ").collect::<Vec<&str>>();
            let col = col[col.len() - 1];
            let col = col.split(" as ").collect::<Vec<&str>>();
            let col = col[col.len() - 1];
            Box::new(col.to_sql_fmt())
        })
        .collect::<Vec<Box<SQLDataTypes>>>();
    let columns = &cols.join(", ");

    let mut query = format!("SELECT {} FROM {}", &columns, &table,);
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

    multithread_execution(SQLImplementation::SQLite(conn_info.to_owned()) ,sqlite_handle_execution, select_props, &header, query, count)
}

pub(crate) fn build_select_sqlite_single_thread(
    select_props: SelectProps,
) -> Result<Vec<Vec<Box<SQLDataTypes>>>, Error> {
    let conn_info = match &select_props.connect {
        SQLImplementation::Oracle(_) => return Err(Error::SQLVariationError),
        SQLImplementation::SQLite(connect) => connect,
    };

    let table = &select_props.table;
    let cols = &select_props
        .columns
        .iter()
        .map(|col| -> Result<String, Error> {
            let col = match col {
                crate::statements::select::Column::Name(name) => {
                    format!("{}.{}", name.table, name.name)
                }
                crate::statements::select::Column::Function(function) => format!("{}", function),
                crate::statements::select::Column::Varchar(varchar) => format!("'{}'", varchar),
                crate::statements::select::Column::ALL(all) => {
                    let columns = conn_info.table_info(&table)?;
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

    let columns = &cols.join(", ");

    let conn = conn_info.initialize_connection()?;

    let mut query = format!("SELECT {} FROM {}", &columns, &table,);

    query = shared_select_operations(&select_props, query)?;

    query = limit_offset(&select_props, query);

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
