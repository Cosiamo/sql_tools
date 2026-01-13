use crate::{
    statements::select::{Column, Direction, JoinType, OrderBy, SelectProps},
};

pub(crate) fn join_operations(select_props: &SelectProps, mut query: String) -> String {
    for join in &select_props.joins {
        let join_type = match join.join_type {
            JoinType::Inner => format!("INNER"),
            JoinType::Outer => format!("OUTER"),
            JoinType::Right => format!("RIGHT"),
            JoinType::Left => format!("LEFT"),
        };
        let join_table = &join.table;
        let primary_column = format!("{}.{}", select_props.table, join.primary_column);
        let foreign_column = format!("{}.{}", join.table, join.foreign_column);
        query =
            format!("{query} {join_type} JOIN {join_table} ON {primary_column} = {foreign_column}");
    }
    query
}

pub(crate) fn filters(select_props: &SelectProps, query: &String) -> String {
    if let Some(filters) = &select_props.clause {
        format!("{} WHERE {}", query, filters)
    } else {
        query.to_owned()
    }
}

pub(crate) fn group_by(group: &Vec<Column>, query: &String) -> String {
    let mut v = Vec::new();
    for group_by in group {
        v.push(group_by.fmt_to_string());
    }
    format!("{} GROUP BY {}", query, v.join(", "))
}

pub(crate) fn order_by(order: &Vec<OrderBy>, query: &String) -> String {
    let mut v = Vec::new();
    for order_by in order {
        match order_by.by {
            Direction::ASC => v.push(format!("{} ASC", order_by.column.fmt_to_string())),
            Direction::DESC => v.push(format!("{} DESC", order_by.column.fmt_to_string())),
        }
    }
    format!("{} ORDER BY {}", query, v.join(", "))
}

pub(crate) fn limit_offset(select_props: &SelectProps, mut query: String) -> String {
    if let Some(limit) = select_props.limit.limit {
        query = format!("{} LIMIT {}", query, limit);
    }
    if let Some(offset) = select_props.limit.offset {
        query = format!("{} OFFSET {}", query, offset);
    }
    query
}

pub(crate) fn limit_offset_oracle(select_props: &SelectProps, mut query: String) -> String {
    if let Some(offset) = select_props.limit.offset {
        query = format!("{} OFFSET {} ROWS", query, offset)
    }
    if let Some(limit) = select_props.limit.limit {
        query = format!("{} FETCH NEXT {} ONLY", query, limit);
    }
    query
}
