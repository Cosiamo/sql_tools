use crate::{
    query_conjunctions::{
        QueryConjunctions, WhereArg,
        utils::{build_conjunction_stmt, build_where_stmt},
    },
    statements::{
        delete::DeleteProps,
        select::{ColumnProps, SelectProps},
        update::UpdateProps,
    },
};

macro_rules! impl_query_conjunctions {
    ($type:ty) => {
        impl QueryConjunctions for $type {
            fn where_in(mut self, column: &ColumnProps, values: WhereArg) -> Self {
                let clause = build_where_stmt(column, values, false);
                self.clause = Some(clause);
                self
            }

            fn where_not(mut self, column: &ColumnProps, values: WhereArg) -> Self {
                let clause = build_where_stmt(column, values, true);
                self.clause = Some(clause);
                self
            }

            fn and(mut self, column: &ColumnProps, values: WhereArg) -> Self {
                let clause = build_conjunction_stmt(column, values, &self.clause, "AND", false);
                self.clause = Some(clause);
                self
            }

            fn or(mut self, column: &ColumnProps, values: WhereArg) -> Self {
                let clause = build_conjunction_stmt(column, values, &self.clause, "OR", false);
                self.clause = Some(clause);
                self
            }

            fn and_not(mut self, column: &ColumnProps, values: WhereArg) -> Self {
                let clause = build_conjunction_stmt(column, values, &self.clause, "AND", true);
                self.clause = Some(clause);
                self
            }

            fn or_not(mut self, column: &ColumnProps, values: WhereArg) -> Self {
                let clause = build_conjunction_stmt(column, values, &self.clause, "OR", true);
                self.clause = Some(clause);
                self
            }
        }
    };
}

impl_query_conjunctions!(SelectProps);
impl_query_conjunctions!(DeleteProps);
impl_query_conjunctions!(UpdateProps);