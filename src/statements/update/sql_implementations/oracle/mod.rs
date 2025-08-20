use crate::{Error, SQLImplementation, data_types::SQLDataTypes, statements::update::UpdateProps};

pub(crate) fn oracle_build_update(update_set: UpdateProps) -> Result<usize, Error> {
    let conn_info = match update_set.connect {
        SQLImplementation::Oracle(oracle_connect) => oracle_connect,
        SQLImplementation::SQLite(_) => return Err(Error::SQLVariationError),
    };

    let set_match_len = &update_set.set_match.len();
    let set_vec = update_set.set_match.iter().enumerate().map(|(idx, set_match)| {
        let fmt_data_types: String;
        if set_match.query {
            fmt_data_types = if let SQLDataTypes::Varchar(val) = &set_match.value {
                format!("({val})") 
            } else {
                return Err(Error::UpdateSetQuery)
            };
        } else {
            fmt_data_types = match &set_match.value {
                SQLDataTypes::Varchar(val) => format!("'{}'", val),
                SQLDataTypes::Number(val) => format!("{}", val),
                SQLDataTypes::Float(val) => format!("{}", val),
                SQLDataTypes::Date(val) => format!("to_date(to_char(to_timestamp('{}', 'YYYY-MM-DD HH24:MI:SS.FF3'), 'YYYY-MM-DD HH24:MI:SS'), 'YYYY-MM-DD HH24:MI:SS')", val),
                SQLDataTypes::NULL => format!("''"),
            };
        }

        if set_match_len == &1 { Ok(format!("SET {} = {}", set_match.column, fmt_data_types)) }
        else if idx == 0 { Ok(format!("SET {} = {},", set_match.column, fmt_data_types)) }
        else if &idx == &(set_match_len - 1) { Ok(format!("{} = {}", set_match.column, fmt_data_types)) }
        else { Ok(format!("{} = {},", set_match.column, fmt_data_types)) }
    }).collect::<Result<Vec<String>, Error>>()?;

    let set = set_vec.join(" ");

    let count_sql: String;
    let query = match update_set.clause {
        Some(filters) => {
            count_sql = format!(
                "SELECT COUNT(*) FROM {}.{} WHERE {}",
                &update_set.table.id, &update_set.table.name, &filters
            );
            format!("UPDATE {}.{} {} WHERE {}", &update_set.table.id, &update_set.table.name, set, filters)
        }
        None => {
            count_sql = format!("SELECT COUNT(*) FROM {}.{}", &update_set.table.id, &update_set.table.name);
            format!("UPDATE {}.{} {}", &update_set.table.id, &update_set.table.name, set)
        }
    };

    let conn: oracle::Connection = oracle::Connection::connect(
        &conn_info.username,
        &conn_info.password,
        &conn_info.connection_string,
    )
    .unwrap();

    let mut count: usize = 0;
    let mut stmt = conn.statement(&count_sql).build()?;
    let stmt_query = stmt.query(&[])?;
    for v in stmt_query {
        let p = v?;
        count = p.get::<usize, usize>(0)?;
    }

    conn.execute(&query, &[]).unwrap();
    conn.commit()?;

    Ok(count)
}

pub fn batch_update_oracle(updates: Vec<UpdateProps>) -> Result<(), Error> {
    let connect = &updates[0].connect;
    // let table = &updates[0].query_type.table;
    let conn_info = match connect {
        SQLImplementation::Oracle(oracle_connect) => oracle_connect,
        SQLImplementation::SQLite(_) => return Err(Error::SQLVariationError),
    };

    let sql = updates.iter().map(|update| {
        // println!("{:#?}", update);
        let set_match_len = &update.set_match.len();
        let set = update.set_match.iter().enumerate().map(|(idx, set_match)| {
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
        match &update.clause {
            Some(clause) => format!("UPDATE {}.{} {} WHERE {}", &update.table.id, &update.table.name, set, clause),
            None => format!("UPDATE {}.{} {}", &update.table.id, &update.table.name, set),
        }
    }).collect::<Vec<String>>().join("; ");

    let query = format!("BEGIN {sql}; END;");

    let conn: oracle::Connection = oracle::Connection::connect(
        &conn_info.username,
        &conn_info.password,
        &conn_info.connection_string,
    )
    .unwrap();
    conn.execute(&query, &[]).unwrap();
    conn.commit()?;

    Ok(())
}
