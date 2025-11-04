use crate::{WhereArg, query_conjunctions::{QueryConjunctions, utils::{conjunction_match, conjunction_match_not, match_table_ids}}, statements::{delete::DeleteProps, select::SelectProps, update::UpdateProps}};

impl QueryConjunctions for SelectProps {
    fn and(mut self, column: &str, values: WhereArg) -> Self {
        let column = match_table_ids(&self.table, column);
        let clause = conjunction_match(&column, values, &self.clause, "AND");
        self.clause = Some(clause);
        self
    }

    fn or(mut self, column: &str, values: WhereArg) -> Self {
        let column = match_table_ids(&self.table, column);
        let clause = conjunction_match(&column, values, &self.clause, "OR");
        self.clause = Some(clause);
        self
    }

    fn and_not(mut self, column: &str, values: WhereArg) -> Self {
        let column = match_table_ids(&self.table, column);
        let clause = conjunction_match_not(&column, values, &self.clause, "AND");
        self.clause = Some(clause);
        self
    }

    fn or_not(mut self, column: &str, values: WhereArg) -> Self {
        let column = match_table_ids(&self.table, column);
        let clause = conjunction_match_not(&column, values, &self.clause, "OR");
        self.clause = Some(clause);
        self
    }
}

impl QueryConjunctions for DeleteProps {
    fn and(mut self, column: &str, values: WhereArg) -> Self {
        let clause = conjunction_match(&column, values, &self.clause, "AND");
        self.clause = Some(clause);
        self
    }

    fn or(mut self, column: &str, values: WhereArg) -> Self {
        let clause = conjunction_match(&column, values, &self.clause, "OR");
        self.clause = Some(clause);
        self
    }

    fn and_not(mut self, column: &str, values: WhereArg) -> Self {
        let clause = conjunction_match_not(&column, values, &self.clause, "AND");
        self.clause = Some(clause);
        self
    }

    fn or_not(mut self, column: &str, values: WhereArg) -> Self {
        let clause = conjunction_match_not(&column, values, &self.clause, "OR");
        self.clause = Some(clause);
        self
    }
}

impl QueryConjunctions for UpdateProps {
    fn and(mut self, column: &str, values: WhereArg) -> Self {
        let clause = conjunction_match(&column, values, &self.clause, "AND");
        self.clause = Some(clause);
        self
    }

    fn or(mut self, column: &str, values: WhereArg) -> Self {
        let clause = conjunction_match(&column, values, &self.clause, "OR");
        self.clause = Some(clause);
        self
    }

    fn and_not(mut self, column: &str, values: WhereArg) -> Self {
        let clause = conjunction_match_not(&column, values, &self.clause, "AND");
        self.clause = Some(clause);
        self
    }

    fn or_not(mut self, column: &str, values: WhereArg) -> Self {
        let clause = conjunction_match_not(&column, values, &self.clause, "OR");
        self.clause = Some(clause);
        self
    }
}