use crate::{clauses::delete::DeleteProps, data_types::ToSQLData};

use super::{WhereClauseBuilder, utils::where_clause_value_format};

impl WhereClauseBuilder for DeleteProps {
    fn and<T: ToSQLData>(mut self, column: &str, values: Vec<T>) -> Self {
        let value = where_clause_value_format(values);
        let and = format!("{column} IN ({value})");
        let clause = if let Some(existing) = self.clause {
            format!("{existing} AND {and}")
        } else {
            format!("{and}")
        };
        self.clause = Some(clause);
        self
    }

    fn or<T: ToSQLData>(mut self, column: &str, values: Vec<T>) -> Self {
        let value = where_clause_value_format(values);
        let or = format!("{column} IN ({value})");
        let clause = if let Some(existing) = self.clause {
            format!("{existing} OR {or}")
        } else {
            format!("{or}")
        };
        self.clause = Some(clause);
        self
    }

    fn and_not<T: ToSQLData>(mut self, column: &str, values: Vec<T>) -> Self {
        let value = where_clause_value_format(values);
        let and = format!("{column} NOT IN ({value})");
        let clause = if let Some(existing) = self.clause {
            format!("{existing} AND {and}")
        } else {
            format!("{and}")
        };
        self.clause = Some(clause);
        self
    }

    fn or_not<T: ToSQLData>(mut self, column: &str, values: Vec<T>) -> Self {
        let value = where_clause_value_format(values);
        let or = format!("{column} NOT IN ({value})");
        let clause = if let Some(existing) = self.clause {
            format!("{existing} OR {or}")
        } else {
            format!("{or}")
        };
        self.clause = Some(clause);
        self
    }

    fn and_null(mut self, column: &str) -> Self {
        let and = format!("{column} IS NULL");
        let clause = if let Some(existing) = self.clause {
            format!("{existing} AND {and}")
        } else {
            format!("{and}")
        };
        self.clause = Some(clause);
        self
    }

    fn and_not_null(mut self, column: &str) -> Self {
        let and = format!("{column} IS NOT NULL");
        let clause = if let Some(existing) = self.clause {
            format!("{existing} AND {and}")
        } else {
            format!("{and}")
        };
        self.clause = Some(clause);
        self
    }

    fn or_null(mut self, column: &str) -> Self {
        let or = format!("{column} IS NULL");
        let clause = if let Some(existing) = self.clause {
            format!("{existing} OR {or}")
        } else {
            format!("{or}")
        };
        self.clause = Some(clause);
        self
    }

    fn or_not_null(mut self, column: &str) -> Self {
        let or = format!("{column} IS NOT NULL");
        let clause = if let Some(existing) = self.clause {
            format!("{existing} OR {or}")
        } else {
            format!("{or}")
        };
        self.clause = Some(clause);
        self
    }
}
