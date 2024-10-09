use crate::{select::SelectProps, update::UpdateSet};

pub mod select;
pub mod update;

pub struct WhereSelect {
    pub query_type: SelectProps,
    pub clause: String,
}

pub struct WhereUpdate {
    pub query_type: UpdateSet,
    pub clause: String,
}

pub trait ClauseBuilder {
    fn and(self, column: &str, value: &str) -> Self;
    fn or(self, column: &str, value: &str) -> Self;
}