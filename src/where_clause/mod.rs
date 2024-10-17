use crate::{data_types::ToSQLData, select::SelectProps, update::UpdateSet};

pub mod select;
pub mod update;
pub mod utils;

pub struct WhereSelect {
    pub query_type: SelectProps,
    pub clause: String,
}

pub struct WhereUpdate {
    pub query_type: UpdateSet,
    pub clause: String,
}

pub trait ClauseBuilder {
    fn and<T: ToSQLData>(self, column: &str, values: Vec<T>) -> Self;
    fn or<T: ToSQLData>(self, column: &str, values: Vec<T>) -> Self;
    fn and_not<T: ToSQLData>(self, column: &str, values: Vec<T>) -> Self;
    fn or_not<T: ToSQLData>(self, column: &str, values: Vec<T>) -> Self;
}