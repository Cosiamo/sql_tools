use crate::{alter::AlterProps, create::CreateProps, data_types::{SQLDataTypes, ToSQLData}, insert::InsertProps, select::{OrderBy, SelectProps}, update::UpdateProps, utils::remove_invalid_chars, QueryBuilder, SQLVariation};

use super::SQLiteConnect;

impl SQLiteConnect {
    pub fn new_path(path: &str) -> Self {
        SQLiteConnect {
            path: path.to_string(),
        }
    }
}

impl QueryBuilder for SQLiteConnect {
    fn select(&self, table: &str, columns: Vec<&str>) -> SelectProps {
        let fmt_cols = columns.iter().map(|cols| { cols.to_string() }).collect::<Vec<String>>();
        SelectProps {
            connect: SQLVariation::SQLite(self.clone()),
            columns: fmt_cols,
            table: table.to_string(),
            clause: None,
            order_by: (None, OrderBy::None),
            group_by: None,
        }
    }

    fn update(&self, table: &str) -> crate::update::UpdateProps {
        UpdateProps {
            connect: SQLVariation::SQLite(self.clone()),
            table: table.to_string(),
        }
    }

    fn insert<T: ToSQLData>(&self, table: &str, data: Vec<Vec<T>>) -> InsertProps {
        let mut grid = data.iter().map(|row|{
            row.iter().map(|cell| cell.fmt_data_borrowed()).collect::<Vec<SQLDataTypes>>()
        }).collect::<Vec<Vec<SQLDataTypes>>>();
        let header = grid[0].iter().map(|cell| {
            let res = format!("{}", cell);
            remove_invalid_chars(&res)
        }).collect::<Vec<String>>();
        grid.remove(0);
        InsertProps {
            connect: SQLVariation::SQLite(self.clone()),
            grid,
            table: table.to_string(),
            header,
        }
    }

    fn create(&self) -> crate::create::CreateProps {
        CreateProps {
            connect: SQLVariation::SQLite(self.clone()),
        }
    }

    fn alter(&self) -> crate::alter::AlterProps {
        AlterProps {
            connect: SQLVariation::SQLite(self.clone()),
        }
    }
}