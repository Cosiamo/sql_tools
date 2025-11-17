use crate::{
    Error,
    statements::select::{
        SelectProps,
        sql_implementations::mutate_query::{filters, group_by, join_operations, order_by},
    },
};

pub(crate) mod multithread;
pub(crate) mod mutate_query;
pub mod oracle;
pub mod sqlite;

pub(crate) fn shared_select_operations(
    select_props: &SelectProps,
    mut query: String,
) -> Result<String, Error> {
    if &select_props.joins.len() > &0 {
        query = join_operations(&select_props, query);
    }

    // ===== Where, and, or =====
    query = filters(&select_props, &query);

    query = group_by(&select_props, &query);

    query = order_by(&select_props, &query)?;

    Ok(query)
}
