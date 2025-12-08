use indicatif::ProgressBar;

use crate::{
    Error, SQLImplementation,
    data_types::SQLDataTypes,
    statements::insert::{
        InsertProps,
        sql_implementations::sqlite::utils::{create_sqlite_table, does_sqlite_table_exist},
    },
};

pub(crate) mod utils;

pub(crate) fn sqlite_build_insert(insert_props: InsertProps) -> Result<(), Error> {
    let conn_info = match &insert_props.connect {
        SQLImplementation::Oracle(_) => return Err(Error::SQLVariationError),
        SQLImplementation::SQLite(connect) => connect,
    };

    let table_exist = does_sqlite_table_exist(&insert_props, &conn_info)?;
    if !table_exist && insert_props.create {
        create_sqlite_table(&insert_props, conn_info)?;
    } else if !table_exist && !insert_props.create {
        return Err(Error::TableDoesNotExist);
    }

    let mut values = Vec::new();
    for idx in 0..insert_props.grid.len() {
        values.push(["?", &(idx + 1).to_string()].concat())
    }

    let conn = conn_info.initialize_connection()?;

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

    does_sqlite_table_exist(&insert_props, conn_info)?;

    let mut values = Vec::new();
    for idx in 0..insert_props.grid.len() {
        values.push(["?", &(idx + 1).to_string()].concat())
    }

    let conn = conn_info.initialize_connection()?;
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
