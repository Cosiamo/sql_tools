use crate::{data_types::ToSQLData, statements::{select::SelectProps, where_clause::utils::match_table_ids}};

use super::{WhereClauseBuilder, utils::where_clause_value_format};

impl WhereClauseBuilder for SelectProps {
    fn and<T: ToSQLData>(mut self, column: &str, values: Vec<T>) -> Self {
        let column = match_table_ids(&self.table, column);
        let value = where_clause_value_format(values);
        let and = format!("{} IN ({})", column, value);
        let clause = if let Some(existing) = self.clause {
            format!("{existing} AND {and}")
        } else {
            format!("{and}")
        };
        self.clause = Some(clause);
        self
    }

    fn or<T: ToSQLData>(mut self, column: &str, values: Vec<T>) -> Self {
        let column = match_table_ids(&self.table, column);
        let value = where_clause_value_format(values);
        let or = format!("{} IN ({})", column, value);
        let clause = if let Some(existing) = self.clause {
            format!("{existing} OR {or}")
        } else {
            format!("{or}")
        };
        self.clause = Some(clause);
        self
    }

    fn and_not<T: ToSQLData>(mut self, column: &str, values: Vec<T>) -> Self {
        let column = match_table_ids(&self.table, column);
        let value = where_clause_value_format(values);
        let and = format!("{} NOT IN ({})", column, value);
        let clause = if let Some(existing) = self.clause {
            format!("{existing} AND {and}")
        } else {
            format!("{and}")
        };
        self.clause = Some(clause);
        self
    }

    fn or_not<T: ToSQLData>(mut self, column: &str, values: Vec<T>) -> Self {
        let column = match_table_ids(&self.table, column);
        let value = where_clause_value_format(values);
        let or = format!("{} NOT IN ({})", column, value);
        let clause = if let Some(existing) = self.clause {
            format!("{existing} OR {or}")
        } else {
            format!("{or}")
        };
        self.clause = Some(clause);
        self
    }

    fn and_null(mut self, column: &str) -> Self {
        let column = match_table_ids(&self.table, column);
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
        let column = match_table_ids(&self.table, column);
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
        let column = match_table_ids(&self.table, column);
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
        let column = match_table_ids(&self.table, column);
        let or = format!("{column} IS NOT NULL");
        let clause = if let Some(existing) = self.clause {
            format!("{existing} OR {or}")
        } else {
            format!("{or}")
        };
        self.clause = Some(clause);
        self
    }
    
    fn and_like(mut self, column: &str, value: &str) -> Self {
        let column = match_table_ids(&self.table, column);
        let and = format!("{column} LIKE '{value}'");
        let clause = if let Some(existing) = self.clause {
            format!("{existing} AND {and}")
        } else {
            format!("{and}")
        };
        self.clause = Some(clause);
        self
    }
    
    fn or_like(mut self, column: &str, value: &str) -> Self {
        let column = match_table_ids(&self.table, column);
        let or = format!("{column} LIKE '{value}'");
        let clause = if let Some(existing) = self.clause {
            format!("{existing} OR {or}")
        } else {
            format!("{or}")
        };
        self.clause = Some(clause);
        self
    }
    
    fn and_not_like(mut self, column: &str, value: &str) -> Self {
        let column = match_table_ids(&self.table, column);
        let and = format!("{column} NOT LIKE '{value}'");
        let clause = if let Some(existing) = self.clause {
            format!("{existing} AND {and}")
        } else {
            format!("{and}")
        };
        self.clause = Some(clause);
        self
    }
    
    fn or_not_like(mut self, column: &str, value: &str) -> Self {
        let column = match_table_ids(&self.table, column);
        let or = format!("{column} NOT LIKE '{value}'");
        let clause = if let Some(existing) = self.clause {
            format!("{existing} OR {or}")
        } else {
            format!("{or}")
        };
        self.clause = Some(clause);
        self
    }
}