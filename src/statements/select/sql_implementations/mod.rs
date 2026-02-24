use crate::{
    Error,
    statements::select::{
        Column, SelectProps,
        sql_implementations::{
            mutate_query::{filters, group_by, join_operations, order_by},
            oracle::columns::get_column_names_oracle,
        },
    },
};

pub(crate) mod multithread;
pub(crate) mod mutate_query;
pub mod oracle;
pub mod sqlite;

pub(crate) fn extract_column_name(col: &str) -> &str {
    col.split('.')
        .last()
        .and_then(|s| s.split(' ').last())
        .and_then(|s| s.split(" as ").last())
        .unwrap_or(col)
}

pub(crate) fn shared_select_operations(
    select_props: &SelectProps,
    mut query: String,
) -> Result<String, Error> {
    if &select_props.joins.len() > &0 {
        query = join_operations(&select_props, query);
    }

    // ===== Where, and, or =====
    query = filters(&select_props, &query);

    if let Some(group) = &select_props.group_by {
        query = group_by(group, &query);
    }

    if let Some(order) = &select_props.order_by {
        query = order_by(order, &query);
    }

    Ok(query)
}

impl SelectProps {
    pub(crate) fn oracle_column_name(&self) -> Result<Vec<String>, Error> {
        self.columns
            .iter()
            .map(|col| -> Result<String, Error> { col.to_query_string(self) })
            .collect::<Result<Vec<String>, Error>>()
    }
}

impl Column {
    pub(crate) fn to_query_string(&self, select_props: &SelectProps) -> Result<String, Error> {
        let col = match self {
            crate::statements::select::Column::Name(name) => {
                format!("{}.{}", name.table, name.name)
            }
            crate::statements::select::Column::Function(function) => {
                format!("{}", function)
            }
            crate::statements::select::Column::Varchar(varchar) => format!("'{}'", varchar),
            crate::statements::select::Column::ALL(all) => {
                let columns = get_column_names_oracle(&select_props)?;
                columns
                    .iter()
                    .map(|col| format!("{}.{}", all, col.name))
                    .collect::<Vec<String>>()
                    .join(", ")
            }
        };
        Ok(col)
    }
}
