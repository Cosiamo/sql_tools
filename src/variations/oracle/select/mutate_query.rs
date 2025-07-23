use crate::{statements::select::{JoinType, OrderBy, SelectProps}, Error};

pub fn join_operations(select_props: &SelectProps, mut query: String) -> String {
    for join in &select_props.joins {
        let join_type = match join.join_type {
            JoinType::Inner => format!("INNER"),
            JoinType::Outer => format!("OUTER"),
            JoinType::Right => format!("RIGHT"),
            JoinType::Left => format!("LEFT"),
        };
        let join_table = join.table.query_fmt();
        let primary_column = format!("{}.{}", select_props.table.id, join.primary_column);
        let foreign_column = format!("{}.{}", join.table.id, join.foreign_column);
        query = format!("{query} {join_type} JOIN {join_table} ON {primary_column} = {foreign_column}");
    }
    query
}

pub fn filters(select_props: &SelectProps, query: &String) -> String {
    if let Some(filters) =  &select_props.clause {
        format!("{} WHERE {}", query, filters)
    } else {
        query.to_owned()
    }
}

pub fn group_by(select_props: &SelectProps, query: &String) -> String {
    if let Some(group_by) = &select_props.group_by {
        format!("{} GROUP BY {}", query, group_by.join(", "))
    } else {
        query.to_owned()
    }
}

pub fn order_by(select_props: &SelectProps, query: &String) -> Result<String, Error> {
    match &select_props.order_by {
        (None, OrderBy::ASC) => return Err(Error::OrderByError),
        (None, OrderBy::DESC) => return Err(Error::OrderByError),
        (None, OrderBy::None) => Ok(query.to_owned()),
        (Some(column), OrderBy::ASC) => Ok(format!("{} ORDER BY {} ASC", query, column)),
        (Some(column), OrderBy::DESC) => Ok(format!("{} ORDER BY {} DESC", query, column)),
        (Some(_), OrderBy::None) => Ok(query.to_owned()),
    }
}

pub fn limit_offset(select_props: &SelectProps, mut query: String) -> String {
    if let Some(offset) = select_props.limit.offset {
        query = format!("{} OFFSET {} ROWS", query, offset)
    }
    if let Some(limit) = select_props.limit.limit {
        query = format!("{} FETCH NEXT {} ONLY", query, limit);
    }
    query
}