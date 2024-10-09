use crate::{errors::Error, select::oracle_sql::utils::stmt_res, SQLTypes};

use super::UpdateSet;

pub fn oracle_build_update(update_set: UpdateSet)  -> Result<(), Error> {
    let conn_info = match update_set.connect {
        SQLTypes::Oracle(oracle_connect) => oracle_connect,
    };
    
    let set_match_len = &update_set.set_match.len();
    let set = update_set.set_match.iter().enumerate().map(|(idx, set_match)| {
        if set_match_len == &1 { format!("SET {} = '{}'", set_match.column, set_match.value) }
        else if idx == 0 { format!("SET {} = '{}',", set_match.column, set_match.value) }
        else if &idx == &(set_match_len - 1) { format!("{} = '{}'", set_match.column, set_match.value) }
        else { format!("{} = '{}',", set_match.column, set_match.value) }
    }).collect::<Vec<String>>().join(" ");
    
    let count_sql: String;
    let query = match update_set.clause {
        Some(filters) => {
            count_sql = format!("SELECT COUNT(*) FROM {} WHERE {}", &update_set.table, &filters);
            format!("UPDATE {} {} WHERE {}", &update_set.table, set, filters)
        },
        None => {
            count_sql = format!("SELECT COUNT(*) FROM {}", &update_set.table);
            format!("UPDATE {} {}", &update_set.table, set)
        },
    };
    println!("{}", query);

    let conn: oracle::Connection = oracle::Connection::connect(&conn_info.username, &conn_info.password, &conn_info.connection_string).unwrap(); 
    // let count_query = conn.query(&count_sql, &[])?;
    let stmt = conn.statement(&count_sql).build()?;
    let count = stmt_res(stmt, 1 as usize)?;
    count.iter().for_each(|c| c.iter().for_each(|t| println!("UPDATED {:?} ROWS", t)));

    Ok(())
}