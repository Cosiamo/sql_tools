use indicatif::ProgressBar;
use rusqlite::Connection;

use crate::{clauses::select::SelectBuilder, data_types::SQLDataTypes, variations::SQLiteConnect, Error, QueryBuilder, SQLVariation};

use super::InsertProps;

pub(crate) fn sqlite_build_insert(insert_props: InsertProps) -> Result<(), Error> {
    let conn_info = match &insert_props.connect {
        SQLVariation::Oracle(_) => return Err(Error::SQLVariationError),
        SQLVariation::SQLite(connect) => connect,
    };

    // Does table exist
    let connection = SQLiteConnect::new_path(&conn_info.path);
    let exists_sql = format!("PRAGMA_TABLE_INFO('{}')", insert_props.table);
    let exists = connection.select(&exists_sql, vec!["name"])
        .build_single_thread().unwrap();

    if exists.len() == 0 { return Err(Error::TableDoesNotExist) }

    let mut values = Vec::new();
    for idx in 0..insert_props.grid.len() {
        values.push(["?", &(idx + 1).to_string()].concat())
    }

    let conn = Connection::open(&conn_info.path)?;

    let mut batch: Vec<String> = Vec::new();
    insert_props.grid.iter().for_each(|row| {
        let values = row.iter().map(|cell| {
            match cell {
                SQLDataTypes::Varchar(val) => format!("'{val}'"),
                SQLDataTypes::Number(val) => format!("{val}"),
                SQLDataTypes::Float(val) => format!("{val}"),
                SQLDataTypes::Date(val) => format!("'{}'", val.to_string()),
                SQLDataTypes::NULL => format!(""),
            }
        }).collect::<Vec<String>>();
        let sql = format!("INSERT INTO {} ({}) VALUES ({})", insert_props.table, insert_props.header.join(", "), values.join(", "));
        batch.push(sql);
    });
    
    let sql = format!("
        BEGIN;
        {};
        COMMIT;
    ", batch.join("; "));
    conn.execute_batch(&sql)?;

    Ok(())
}

pub(crate) fn sqlite_build_insert_pb(insert_props: InsertProps) -> Result<(), Error> {
    let conn_info = match &insert_props.connect {
        SQLVariation::Oracle(_) => return Err(Error::SQLVariationError),
        SQLVariation::SQLite(connect) => connect,
    };

    // Does table exist
    let connection = SQLiteConnect::new_path(&conn_info.path);
    let exists_sql = format!("PRAGMA_TABLE_INFO('{}')", insert_props.table);
    let exists = connection.select(&exists_sql, vec!["name"])
        .build_single_thread().unwrap();

    if exists.len() == 0 { return Err(Error::TableDoesNotExist) }

    let mut values = Vec::new();
    for idx in 0..insert_props.grid.len() {
        values.push(["?", &(idx + 1).to_string()].concat())
    }

    let conn = Connection::open(&conn_info.path)?;
    let progress_bar = ProgressBar::new(*&insert_props.grid.len() as u64);

    let mut batch: Vec<String> = Vec::new();
    insert_props.grid.iter().for_each(|row| {
        let values = row.iter().map(|cell| {
            match cell {
                SQLDataTypes::Varchar(val) => format!("'{val}'"),
                SQLDataTypes::Number(val) => format!("{val}"),
                SQLDataTypes::Float(val) => format!("{val}"),
                SQLDataTypes::Date(val) => format!("'{}'", val.to_string()),
                SQLDataTypes::NULL => format!(""),
            }
        }).collect::<Vec<String>>();
        let sql = format!("INSERT INTO {} ({}) VALUES ({})", insert_props.table, insert_props.header.join(", "), values.join(", "));
        batch.push(sql);
        progress_bar.inc(1u64);
    });
    
    let sql = format!("
        BEGIN;
        {};
        COMMIT;
    ", batch.join("; "));
    conn.execute_batch(&sql)?;

    Ok(())
}