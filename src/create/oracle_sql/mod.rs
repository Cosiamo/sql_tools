use crate::{errors::Error, SQLTypes};

use super::{CreateDataTypes, CreateProps};

pub fn oracle_build_create_table(create_props: CreateProps) -> Result<(), Error> {
    let conn_info = match create_props.connect {
        SQLTypes::Oracle(oracle_connect) => oracle_connect,
    };

    let cols_and_data_types = create_props.columns.iter().map(|col_props|{
        match col_props.data_type {
            CreateDataTypes::VARCHAR(num) => format!("{} VARCHAR2({})", &col_props.name, num),
            CreateDataTypes::INT => format!("{} NUMBER", &col_props.name),
            CreateDataTypes::FLOAT => format!("{} FLOAT", &col_props.name),
            CreateDataTypes::DATE => format!("{} DATE", &col_props.name),
        }
    }).collect::<Vec<String>>().join(", ");

    let sql = format!("CREATE TABLE {} ({})", create_props.table, cols_and_data_types);
    // println!("{}", sql);
    let conn: oracle::Connection = oracle::Connection::connect(&conn_info.username, &conn_info.password, &conn_info.connection_string).unwrap(); 
    conn.execute(&sql, &[])?;
    conn.commit()?;
    Ok(())
}