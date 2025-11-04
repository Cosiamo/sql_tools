use crate::{
    Error, SQLImplementation, data_types::SQLDataTypes, query_conjunctions::utils::where_clause_value_format, statements::select::{
            JoinType, Joins, WhereArg, sql_implementations::{
                oracle::{oracle_build_select, oracle_build_single_thread_select},
                sqlite::{build_select_sqlite, build_select_sqlite_single_thread},
            }
        }
};

use super::{Limit, OrderBy, Ordered, SelectBuilder, SelectProps, group_by::Grouped};

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

    fn where_in(mut self, column: &str, values: WhereArg) -> Self {
        let where_clause = match values {
            WhereArg::Values(items) => {
                let value = where_clause_value_format(items);
                let column = match_table_ids(&self.table, column);
                format!("{column} IN ({value})")
            },
            WhereArg::Like(like) => {
                let column = match_table_ids(&self.table, column);
                format!("{column} LIKE '{like}'")
            },
            WhereArg::Query(value) => {
                format!("{column} IN ({value})")
            },
            WhereArg::NULL => {
                let column = match_table_ids(&self.table, column);
                format!("{column} IS NULL")
            },
        };
        self.clause = Some(where_clause);
        self
    }

    fn where_not(mut self, column: &str, values: WhereArg) -> Self {
        let where_clause = match values {
            WhereArg::Values(items) => {
                let value = where_clause_value_format(items);
                let column = match_table_ids(&self.table, column);
                format!("{column} NOT IN ({value})")
            },
            WhereArg::Like(like) => {
                let column = match_table_ids(&self.table, column);
                format!("{column} NOT LIKE '{like}'")
            },
            WhereArg::Query(value) => {
                format!("{column} NOT IN ({value})")
            },
            WhereArg::NULL => {
                let column = match_table_ids(&self.table, column);
                format!("{column} IS NOT NULL")
            },
        };
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

impl Ordered {
    pub fn build(self) -> Result<Vec<Vec<Box<SQLDataTypes>>>, Error> {
        self.select.build()
    }

    pub fn build_single_thread(self) -> Result<Vec<Vec<Box<SQLDataTypes>>>, Error> {
        self.select.build_single_thread()
    }
}

fn match_table_ids(table: &str, column: &str) -> String {
    if column.contains(".") {
        column.to_owned()
    } else {
        format!("{table}.{column}")
    }
}
