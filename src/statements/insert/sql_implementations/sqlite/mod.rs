use indicatif::ProgressBar;
use rusqlite::Connection;

use crate::{
    Error, QueryBuilder, SQLImplementation,
    data_types::SQLDataTypes,
    sql_implementations::SQLiteConnect,
    statements::{
        create::{CreateColumns, CreateDataTypes, ModifyCreateTable},
        insert::{InsertProps, sql_implementations::oracle::validation::get_col_indexes},
        select::SelectBuilder,
    },
};

pub(crate) fn sqlite_build_insert(insert_props: InsertProps) -> Result<(), Error> {
    let conn_info = match &insert_props.connect {
        SQLImplementation::Oracle(_) => return Err(Error::SQLVariationError),
        SQLImplementation::SQLite(connect) => connect,
    };

    // Does table exist
    let connection = SQLiteConnect::new_path(&conn_info.path);
    let table = &insert_props.table.split(".").collect::<Vec<&str>>();
    let exists = connection
        .select("sqlite_master", vec!["name"])
        .where_in("name", vec![table[0]])
        .build_single_thread()
        .unwrap();

    if exists.len() == 0 && !insert_props.create {
        return Err(Error::TableDoesNotExist);
    } else if exists.len() == 0 && insert_props.create {
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
    }

    let mut values = Vec::new();
    for idx in 0..insert_props.grid.len() {
        values.push(["?", &(idx + 1).to_string()].concat())
    }

    let conn = Connection::open(&conn_info.path)?;

    let mut batch: Vec<String> = Vec::new();
    insert_props.grid.iter().for_each(|row| {
        let values = row
            .iter()
            .map(|cell| match cell {
                SQLDataTypes::Varchar(val) => format!("'{val}'"),
                SQLDataTypes::Number(val) => format!("{val}"),
                SQLDataTypes::Float(val) => format!("{val}"),
                SQLDataTypes::Date(val) => format!("'{}'", val.to_string()),
                SQLDataTypes::NULL => format!("NULL"),
            })
            .collect::<Vec<String>>();
        let sql = format!(
            "INSERT INTO {} ({}) VALUES ({})",
            insert_props.table,
            insert_props.header.join(", "),
            values.join(", ")
        );
        batch.push(sql);
    });

    let sql = format!(
        "
        BEGIN;
        {};
        COMMIT;
    ",
        batch.join("; ")
    );
    conn.execute_batch(&sql)?;

    Ok(())
}

pub(crate) fn sqlite_build_insert_pb(insert_props: InsertProps) -> Result<(), Error> {
    let conn_info = match &insert_props.connect {
        SQLImplementation::Oracle(_) => return Err(Error::SQLVariationError),
        SQLImplementation::SQLite(connect) => connect,
    };

    // Does table exist
    let connection = SQLiteConnect::new_path(&conn_info.path);
    let table = &insert_props.table.split(".").collect::<Vec<&str>>();
    let exists = connection
        .select("sqlite_master", vec!["name"])
        .where_in("name", vec![table[0]])
        .build_single_thread()
        .unwrap();

    if exists.len() == 0 && !insert_props.create {
        return Err(Error::TableDoesNotExist);
    } else if exists.len() == 0 && insert_props.create {
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
    }

    let mut values = Vec::new();
    for idx in 0..insert_props.grid.len() {
        values.push(["?", &(idx + 1).to_string()].concat())
    }

    let conn = Connection::open(&conn_info.path)?;
    let progress_bar = ProgressBar::new(*&insert_props.grid.len() as u64);

    let mut batch: Vec<String> = Vec::new();
    insert_props.grid.iter().for_each(|row| {
        let values = row
            .iter()
            .map(|cell| match cell {
                SQLDataTypes::Varchar(val) => format!("'{val}'"),
                SQLDataTypes::Number(val) => format!("{val}"),
                SQLDataTypes::Float(val) => format!("{val}"),
                SQLDataTypes::Date(val) => format!("'{}'", val.to_string()),
                SQLDataTypes::NULL => format!(""),
            })
            .collect::<Vec<String>>();
        let sql = format!(
            "INSERT INTO {} ({}) VALUES ({})",
            insert_props.table,
            insert_props.header.join(", "),
            values.join(", ")
        );
        batch.push(sql);
        progress_bar.inc(1u64);
    });

    let sql = format!(
        "
        BEGIN;
        {};
        COMMIT;
    ",
        batch.join("; ")
    );
    conn.execute_batch(&sql)?;

    Ok(())
}
