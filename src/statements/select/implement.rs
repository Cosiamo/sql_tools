use crate::{
    Error, SQLImplementation,
    data_types::SQLDataTypes,
    statements::select::{
        Column, JoinType, Joins, sql_implementations::{
            oracle::{oracle_build_select, oracle_build_single_thread_select},
            sqlite::{build_select_sqlite, build_select_sqlite_single_thread},
        }
    },
};

use super::{Limit, OrderBy, SelectBuilder, SelectProps};

impl SelectBuilder for SelectProps {
    fn inner_join(
        mut self,
        foreign_table: &str,
        primary_column: &str,
        foreign_column: &str,
    ) -> Self {
        let join = Joins {
            table: foreign_table.to_owned(),
            primary_column: String::from(primary_column),
            foreign_column: String::from(foreign_column),
            join_type: JoinType::Inner,
        };
        self.joins.push(join);
        self
    }

    fn outer_join(
        mut self,
        foreign_table: &str,
        primary_column: &str,
        foreign_column: &str,
    ) -> Self {
        let join = Joins {
            table: foreign_table.to_owned(),
            primary_column: String::from(primary_column),
            foreign_column: String::from(foreign_column),
            join_type: JoinType::Outer,
        };
        self.joins.push(join);
        self
    }

    fn right_join(
        mut self,
        foreign_table: &str,
        primary_column: &str,
        foreign_column: &str,
    ) -> Self {
        let join = Joins {
            table: foreign_table.to_owned(),
            primary_column: String::from(primary_column),
            foreign_column: String::from(foreign_column),
            join_type: JoinType::Right,
        };
        self.joins.push(join);
        self
    }

    fn left_join(
        mut self,
        foreign_table: &str,
        primary_column: &str,
        foreign_column: &str,
    ) -> Self {
        let join = Joins {
            table: foreign_table.to_owned(),
            primary_column: String::from(primary_column),
            foreign_column: String::from(foreign_column),
            join_type: JoinType::Left,
        };
        self.joins.push(join);
        self
    }

    fn order_by(mut self, columns: Vec<OrderBy>) -> Self {
        self.order_by = Some(columns);
        self
    }

    fn group_by(mut self, columns: Vec<&Column>) -> Self {
        let mut group_by = Vec::new();
        for col in columns {
            group_by.push(col.to_owned());
        }
        self.group_by = Some(group_by);
        self
    }

    fn build(self) -> Result<Vec<Vec<Box<SQLDataTypes>>>, Error> {
        match self.connect {
            SQLImplementation::Oracle(_) => oracle_build_select(self),
            SQLImplementation::SQLite(_) => build_select_sqlite(self),
        }
    }

    fn build_single_thread(self) -> Result<Vec<Vec<Box<SQLDataTypes>>>, Error> {
        match self.connect {
            SQLImplementation::Oracle(_) => oracle_build_single_thread_select(self),
            SQLImplementation::SQLite(_) => build_select_sqlite_single_thread(self),
        }
    }

    fn limit(mut self, limit: usize, offset: Option<usize>) -> Self {
        self.limit = Limit {
            limit: Some(limit),
            offset,
        };
        self
    }

    fn return_header(mut self) -> Self {
        self.return_header = true;
        self
    }
}
