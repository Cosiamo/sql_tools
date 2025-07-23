use crate::{statements::select::{OrderBy, SelectProps}, Error};

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