use chrono::NaiveDateTime;
use crate::{Error, statements::update::UpdateProps};
use super::build_set_clause;

fn sqlite_date_fmt(val: &NaiveDateTime) -> String {
    format!("'{}'", val)
}

pub(crate) fn sqlite_build_update(update_set: UpdateProps) -> Result<usize, Error> {
    let conn_info = update_set.connect.as_sqlite()?;

    let set = build_set_clause(&update_set.set_match, sqlite_date_fmt, false)?;

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

    let conn = conn_info.initialize_connection()?;
    conn.execute(&query, [])?;
    let mut stmt = conn.prepare(&count_sql)?;
    let mut rows = stmt.query([])?;
    let mut res: Vec<usize> = Vec::new();
    while let Some(row) = rows.next()? {
        res.push(row.get(0).unwrap())
    }

    if res.len() == 0 {
        return Err(Error::CountError);
    }

    Ok(res[0])
}

pub fn batch_update_sqlite(updates: Vec<UpdateProps>) -> Result<(), Error> {
    // let table = &updates[0].query_type.table;
    let conn_info = updates[0].connect.as_sqlite()?;

    let sql = updates
        .iter()
        .map(|update| {
            let set = build_set_clause(&update.set_match, sqlite_date_fmt, false)?;
            Ok(match &update.clause {
                Some(clause) => format!("UPDATE {} {} WHERE {}", &update.table, set, clause),
                None => format!("UPDATE {} {}", &update.table, set),
            })
        })
        .collect::<Result<Vec<String>, Error>>()?
        .join("; ");

    let query = format!("BEGIN; {sql}; COMMIT;");

    let conn = conn_info.initialize_connection()?;
    conn.execute_batch(&query)?;

    Ok(())
}
