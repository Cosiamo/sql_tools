use crate::{statements::select::{implementations::mutate_query::{filters, group_by, join_operations, order_by}, SelectProps}, Error};

pub mod oracle;
pub mod sqlite;
pub mod mutate_query;

pub(crate) fn shared_select_operations(select_props: &SelectProps, mut query: String) -> Result<String, Error> {
    // ===== Joins =====
    if &select_props.joins.len() > &0 {
        query = join_operations(&select_props, query);
    }

    // ===== If filters =====
    query = filters(&select_props, &query);

    // ===== Group By =====
    query = group_by(&select_props, &query);

    // ===== Order By =====
    query = order_by(&select_props, &query)?;

    Ok(query)
}