use rusqlite::Connection;

use crate::{data_types::SQLDataTypes, select::OrderBy, Error, SQLVariation};

use super::SelectProps;

pub(crate) fn build_select_sqlite(select_props: SelectProps) -> Result<Vec<Vec<SQLDataTypes>>, Error> {
    let conn_info = match &select_props.connect {
        SQLVariation::Oracle(_) => return Err(Error::SQLVariationError),
        SQLVariation::SQLite(connect) => connect,
    };
    
    let conn = Connection::open(&conn_info.path.clone())?;
    
    let mut query = match select_props.clause {
        Some(filters) => format!("SELECT {} FROM {} WHERE {}", &select_props.columns.join(", "), &select_props.table, filters),
        None => format!("SELECT {} FROM {}", &select_props.columns.join(", "), &select_props.table),
    };

    query = if let Some(group_by) = select_props.group_by { format!("{} GROUP BY {}", query, group_by.join(", ")) } else { query };

    match select_props.order_by {
        (None, OrderBy::ASC) => return Err(Error::OrderByError),
        (None, OrderBy::DESC) => return Err(Error::OrderByError),
        (None, OrderBy::None) => query = query,
        (Some(column), OrderBy::ASC) => query = format!("{} ORDER BY {} ASC", query, column),
        (Some(column), OrderBy::DESC) => query = format!("{} ORDER BY {} DESC", query, column),
        (Some(_), OrderBy::None) => query = query,
    }

    let mut stmt = conn.prepare(&query)?;
    let mut rows = stmt.query([])?;
    let mut res = Vec::new();
    while let Some(row) = rows.next()? {
        let p = select_props.columns.iter().enumerate().map(|(idx, _)| {
            row.get::<usize, SQLDataTypes>(idx).unwrap()
        }).collect::<Vec<SQLDataTypes>>();
        res.push(p)
    }

    Ok(res)
}