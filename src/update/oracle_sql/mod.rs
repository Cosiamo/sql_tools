use crate::{data_types::SQLDataTypes, errors::Error, SQLVariation};

use super::UpdateSet;

pub fn oracle_build_update(update_set: UpdateSet)  -> Result<(), Error> {
    let conn_info = match update_set.connect {
        SQLVariation::Oracle(oracle_connect) => oracle_connect,
    };
    
    let set_match_len = &update_set.set_match.len();
    let set = update_set.set_match.iter().enumerate().map(|(idx, set_match)| {
        let fmt_data_types = match &set_match.value {
            SQLDataTypes::VARCHAR(val) => format!("'{}'", val),
            SQLDataTypes::INT(val) => format!("{}", val),
            SQLDataTypes::FLOAT(val) => format!("{}", val),
            SQLDataTypes::DATE(val) => format!("to_date(to_char(to_timestamp('{}', 'YYYY-MM-DD HH24:MI:SS.FF3'), 'YYYY-MM-DD HH24:MI:SS'), 'YYYY-MM-DD HH24:MI:SS')", val),
            SQLDataTypes::NULL => format!("''"),
        };

        if set_match_len == &1 { format!("SET {} = {}", set_match.column, fmt_data_types) }
        else if idx == 0 { format!("SET {} = {},", set_match.column, fmt_data_types) }
        else if &idx == &(set_match_len - 1) { format!("{} = {}", set_match.column, fmt_data_types) }
        else { format!("{} = {},", set_match.column, fmt_data_types) }
    }).collect::<Vec<String>>().join(" ");
    
    // let count_sql: String;
    let query = match update_set.clause {
        Some(filters) => {
            // count_sql = format!("SELECT COUNT(*) FROM {} WHERE {}", &update_set.table, &filters);
            format!("UPDATE {} {} WHERE {}", &update_set.table, set, filters)
        },
        None => {
            // count_sql = format!("SELECT COUNT(*) FROM {}", &update_set.table);
            format!("UPDATE {} {}", &update_set.table, set)
        },
    };

    let conn: oracle::Connection = oracle::Connection::connect(&conn_info.username, &conn_info.password, &conn_info.connection_string).unwrap(); 
    conn.execute(&query, &[]).unwrap();
    conn.commit()?;
    
    // let stmt = conn.statement(&count_sql).build()?;
    // let count = stmt_res(stmt, 1 as usize)?;
    // count.iter().for_each(|c| c.iter().for_each(|t| println!("UPDATED {:?} ROWS", t)));

    Ok(())
}