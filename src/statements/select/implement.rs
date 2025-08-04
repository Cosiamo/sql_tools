use crate::{
    Error, SQLVariation,
    data_types::{SQLDataTypes, ToSQLData},
    statements::{
        select::{
            JoinType, Joins, Table,
            sql_implementations::{
                oracle::{oracle_build_select, oracle_build_single_thread_select},
                sqlite::{build_select_sqlite, build_select_sqlite_single_thread},
            },
        },
        where_clause::utils::where_clause_value_format,
    },
};

use super::{Limit, OrderBy, Ordered, SelectBuilder, SelectProps, group_by::Grouped};

impl SelectBuilder for SelectProps {
    fn inner_join(
        mut self,
        foreign_table: Table,
        primary_column: &str,
        foreign_column: &str,
    ) -> Self {
        let join = Joins {
            table: foreign_table,
            primary_column: String::from(primary_column),
            foreign_column: String::from(foreign_column),
            join_type: JoinType::Inner,
        };
        self.joins.push(join);
        self
    }

    fn outer_join(
        mut self,
        foreign_table: Table,
        primary_column: &str,
        foreign_column: &str,
    ) -> Self {
        let join = Joins {
            table: foreign_table,
            primary_column: String::from(primary_column),
            foreign_column: String::from(foreign_column),
            join_type: JoinType::Outer,
        };
        self.joins.push(join);
        self
    }

    fn right_join(
        mut self,
        foreign_table: Table,
        primary_column: &str,
        foreign_column: &str,
    ) -> Self {
        let join = Joins {
            table: foreign_table,
            primary_column: String::from(primary_column),
            foreign_column: String::from(foreign_column),
            join_type: JoinType::Right,
        };
        self.joins.push(join);
        self
    }

    fn left_join(
        mut self,
        foreign_table: Table,
        primary_column: &str,
        foreign_column: &str,
    ) -> Self {
        let join = Joins {
            table: foreign_table,
            primary_column: String::from(primary_column),
            foreign_column: String::from(foreign_column),
            join_type: JoinType::Left,
        };
        self.joins.push(join);
        self
    }

    fn where_in<T: ToSQLData>(mut self, column: &str, values: Vec<T>) -> Self {
        let column = match_table_ids(&self.table.id, column);
        let value = where_clause_value_format(values);
        let where_clause = format!("{column} IN ({value})");
        self.clause = Some(where_clause);
        self
    }

    fn where_not<T: ToSQLData>(mut self, column: &str, values: Vec<T>) -> Self {
        let column = match_table_ids(&self.table.id, column);
        let value = where_clause_value_format(values);
        let where_clause = format!("{column} NOT IN ({value})");
        self.clause = Some(where_clause);
        self
    }

    fn where_null(mut self, column: &str) -> Self {
        let column = match_table_ids(&self.table.id, column);
        let where_clause = format!("{column} IS NULL");
        self.clause = Some(where_clause);
        self
    }

    fn where_not_null(mut self, column: &str) -> Self {
        let column = match_table_ids(&self.table.id, column);
        let where_clause = format!("{column} IS NOT NULL");
        self.clause = Some(where_clause);
        self
    }

    fn order_asc(mut self, column: &str) -> Ordered {
        self.order_by = (Some(column.to_string()), OrderBy::ASC);
        Ordered { select: self }
    }

    fn order_desc(mut self, column: &str) -> Ordered {
        self.order_by = (Some(column.to_string()), OrderBy::DESC);
        Ordered { select: self }
    }

    fn group_by(mut self, columns: Vec<&str>) -> Grouped {
        self.group_by = Some(
            columns
                .iter()
                .map(|col| col.to_string())
                .collect::<Vec<String>>(),
        );
        Grouped { select: self }
    }

    fn build(self) -> Result<Vec<Vec<Box<SQLDataTypes>>>, Error> {
        match self.connect {
            SQLVariation::Oracle(_) => oracle_build_select(self),
            SQLVariation::SQLite(_) => build_select_sqlite(self),
        }
    }

    fn build_single_thread(self) -> Result<Vec<Vec<Box<SQLDataTypes>>>, Error> {
        match self.connect {
            SQLVariation::Oracle(_) => oracle_build_single_thread_select(self),
            SQLVariation::SQLite(_) => build_select_sqlite_single_thread(self),
        }
    }

    fn limit(mut self, limit: usize, offset: Option<usize>) -> Self {
        self.limit = Limit {
            limit: Some(limit),
            offset,
        };
        self
    }
}

impl Ordered {
    pub fn build(self) -> Result<Vec<Vec<Box<SQLDataTypes>>>, Error> {
        self.select.build()
    }

    pub fn build_single_thread(self) -> Result<Vec<Vec<Box<SQLDataTypes>>>, Error> {
        self.select.build_single_thread()
    }
}

fn match_table_ids(id: &String, column: &str) -> String {
    if column.contains(".") {
        column.to_owned()
    } else {
        format!("{id}.{column}")
    }
}
