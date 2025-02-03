use rusqlite::Connection;

use crate::{clauses::{update::UpdateSet, where_clause::WhereUpdate}, data_types::SQLDataTypes, Error, SQLVariation};

pub(crate) fn sqlite_build_update(update_set: UpdateSet)  -> Result<usize, Error> {
    let conn_info = match &update_set.connect {
        SQLVariation::Oracle(_) => return Err(Error::SQLVariationError),
        SQLVariation::SQLite(connect) => connect,
    };

    let set_match_len = &update_set.set_match.len();
    let set = update_set.set_match.iter().enumerate().map(|(idx, set_match)| {
        let fmt_data_types = match &set_match.value {
            SQLDataTypes::Varchar(val) => format!("'{}'", val),
            SQLDataTypes::Number(val) => format!("{}", val),
            SQLDataTypes::Float(val) => format!("{}", val),
            SQLDataTypes::Date(val) => format!("to_date(to_char(to_timestamp('{}', 'YYYY-MM-DD HH24:MI:SS.FF3'), 'YYYY-MM-DD HH24:MI:SS'), 'YYYY-MM-DD HH24:MI:SS')", val),
            SQLDataTypes::NULL => format!("''"),
        };

        if set_match_len == &1 { format!("SET {} = {}", set_match.column, fmt_data_types) }
        else if idx == 0 { format!("SET {} = {},", set_match.column, fmt_data_types) }
        else if &idx == &(set_match_len - 1) { format!("{} = {}", set_match.column, fmt_data_types) }
        else { format!("{} = {},", set_match.column, fmt_data_types) }
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
    
    let conn = Connection::open(&conn_info.path.clone())?;
    conn.execute(&query, [])?;
    let mut stmt = conn.prepare(&count_sql)?;
    let mut rows = stmt.query([])?;
    let mut res: Vec<usize> = Vec::new();
    while let Some(row) = rows.next()? {
        res.push(row.get(0).unwrap())
    }

    if res.len() == 0 { return Err(Error::CountError); }

    Ok(res[0])
}

pub fn batch_update_sqlite(updates: Vec<WhereUpdate>) -> Result<(), Error> {
    let connect = &updates[0].query_type.connect;
    // let table = &updates[0].query_type.table;
    let conn_info = match connect {
        SQLVariation::Oracle(_) => return Err(Error::SQLVariationError),
        SQLVariation::SQLite(connect) => connect,
    };

    let sql = updates.iter().map(|update| {
        let set_match_len = &update.query_type.set_match.len();
        let set = update.query_type.set_match.iter().enumerate().map(|(idx, set_match)| {
            let fmt_data_types = match &set_match.value {
                SQLDataTypes::Varchar(val) => format!("'{}'", val),
                SQLDataTypes::Number(val) => format!("{}", val),
                SQLDataTypes::Float(val) => format!("{}", val),
                SQLDataTypes::Date(val) => format!("to_date(to_char(to_timestamp('{}', 'YYYY-MM-DD HH24:MI:SS.FF3'), 'YYYY-MM-DD HH24:MI:SS'), 'YYYY-MM-DD HH24:MI:SS')", val),
                SQLDataTypes::NULL => format!("''"),
            };
    
            if set_match_len == &1 { format!("SET {} = {}", set_match.column, fmt_data_types) }
            else if idx == 0 { format!("SET {} = {},", set_match.column, fmt_data_types) }
            else if &idx == &(set_match_len - 1) { format!("{} = {}", set_match.column, fmt_data_types) }
            else { format!("{} = {},", set_match.column, fmt_data_types) }
        }).collect::<Vec<String>>().join(" ");
        format!("UPDATE {} {} WHERE {}", &update.query_type.table, set, &update.clause)
    }).collect::<Vec<String>>().join("; ");

    let query = format!("BEGIN; {sql}; COMMIT;");

    let conn = Connection::open(&conn_info.path)?; 
    conn.execute_batch(&query)?;

    Ok(())
}