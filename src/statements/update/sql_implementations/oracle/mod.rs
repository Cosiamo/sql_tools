use chrono::NaiveDateTime;
use crate::{Error, SQLImplementation, statements::update::UpdateProps};
use super::build_set_clause;

fn oracle_date_fmt(val: &NaiveDateTime) -> String {
    format!(
        "to_date(to_char(to_timestamp('{}', 'YYYY-MM-DD HH24:MI:SS.FF3'), 'YYYY-MM-DD HH24:MI:SS'), 'YYYY-MM-DD HH24:MI:SS')",
        val
    )
}

pub(crate) fn oracle_build_update(update_set: UpdateProps) -> Result<usize, Error> {
    let conn_info = match update_set.connect {
        SQLImplementation::Oracle(oracle_connect) => oracle_connect,
        SQLImplementation::SQLite(_) => return Err(Error::SQLVariationError),
    };

    let set = build_set_clause(&update_set.set_match, oracle_date_fmt, true)?;

    let count_sql: String;
    let query = match update_set.clause {
        Some(filters) => {
            count_sql = format!(
                "SELECT COUNT(*) FROM {} WHERE {}",
                &update_set.table, &filters
            );
            format!("UPDATE {} {} WHERE {}", &update_set.table, set, filters)
        }
        None => {
            count_sql = format!("SELECT COUNT(*) FROM {}", &update_set.table);
            format!("UPDATE {} {}", &update_set.table, set)
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

    dbg!(&query);
    conn.execute(&query, &[])?;
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

    let sql = updates
        .iter()
        .map(|update| {
            let set = build_set_clause(&update.set_match, oracle_date_fmt, false)?;
            Ok(match &update.clause {
                Some(clause) => format!("UPDATE {} {} WHERE {}", &update.table, set, clause),
                None => format!("UPDATE {} {}", &update.table, set),
            })
        })
        .collect::<Result<Vec<String>, Error>>()?
        .join("; ");

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
