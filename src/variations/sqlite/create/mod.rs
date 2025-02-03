use rusqlite::Connection;

use crate::{clauses::create::{CreateDataTypes, CreateTable}, Error, SQLVariation};

pub(crate) fn sqlite_build_create_table(create_table: CreateTable) -> Result<(), Error> {
    let conn_info = match &create_table.connect {
        SQLVariation::Oracle(_) => return Err(Error::SQLVariationError),
        SQLVariation::SQLite(connect) => connect,
    };

    let cols_and_data_types = create_table.columns.iter().map(|col_props|{
        match col_props.data_type {
            CreateDataTypes::VARCHAR(mut num) => {
                if num == 0 { num = 1 } else { num = num }
                format!("{} VARCHAR2({})", &col_props.name, num)
            },
            CreateDataTypes::NUMBER => format!("{} NUMBER", &col_props.name),
            CreateDataTypes::FLOAT => format!("{} FLOAT", &col_props.name),
            CreateDataTypes::DATE => format!("{} DATE", &col_props.name),
        }
    }).collect::<Vec<String>>().join(", ");

    let sql = format!("CREATE TABLE {} ({})", create_table.table, cols_and_data_types);
    let conn = Connection::open(&conn_info.path)?;
    conn.execute(&sql, ())?;
    Ok(())
}