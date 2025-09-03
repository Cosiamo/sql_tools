use crate::{
    data_types::{SQLDataTypes, ToSQLData}, statements::{
        alter::AlterProps,
        create::CreateProps,
        delete::DeleteProps,
        insert::InsertProps,
        select::{Limit, OrderBy, SelectBuilder, SelectProps},
        update::UpdateInitialization,
    }, utils::remove_invalid_chars, Error, QueryBuilder, SQLImplementation, Table
};

use super::SQLiteConnect;

impl SQLiteConnect {
    pub fn new_path(path: &str) -> Self {
        SQLiteConnect {
            path: path.to_string(),
        }
    }

    pub fn table_info(&self, table: &str) -> Result<Vec<String>, Error> {
        if self.path == "" {
            return Err(Error::TableDoesNotExist);
        };

        let connection = SQLiteConnect::new_path(&self.path);
        let exists_sql = format!("PRAGMA_TABLE_INFO('{}')", table);
        let exists = connection
            .select(&exists_sql, vec!["name"])
            .build_single_thread()?;

        if exists.len() == 0 {
            return Err(Error::TableDoesNotExist);
        };

        let res = exists
            .iter()
            .map(|row| {
                row.iter()
                    .map(|cell| cell.to_string())
                    .collect::<Vec<String>>()
                    .join("")
            })
            .collect::<Vec<String>>();
        Ok(res)
    }
}

impl QueryBuilder for SQLiteConnect {
    fn select(&self, table: &str, columns: Vec<&str>) -> SelectProps {
        let table_name = table.trim();
        let table = Table::new(table_name);

        let fmt_cols = columns
            .iter()
            .map(|cols| {
                if cols.contains(".") || cols == &"*" {
                    cols.to_string()
                } else {
                    format!("{}.{cols}", table.id)
                }
            })
            .collect::<Vec<String>>();

        SelectProps {
            connect: SQLImplementation::SQLite(self.clone()),
            columns: fmt_cols,
            table,
            joins: vec![],
            clause: None,
            order_by: (None, OrderBy::None),
            group_by: None,
            limit: Limit {
                limit: None,
                offset: None,
            },
        }
    }

    fn update(&self, table: &Table) -> UpdateInitialization {
        UpdateInitialization {
            connect: SQLImplementation::SQLite(self.clone()),
            table: table.clone(),
        }
    }

    fn insert<T: ToSQLData>(&self, table: &str, data: Vec<Vec<T>>) -> Result<InsertProps, Error> {
        if data.len() < 2 {
            return Err(Error::NoHeading);
        }
        let mut grid = data
            .iter()
            .map(|row| {
                row.iter()
                    .map(|cell| cell.fmt_data_borrowed())
                    .collect::<Vec<SQLDataTypes>>()
            })
            .collect::<Vec<Vec<SQLDataTypes>>>();
        let header = grid[0]
            .iter()
            .map(|cell| {
                let res = format!("{}", cell);
                remove_invalid_chars(&res)
            })
            .collect::<Vec<String>>();
        grid.remove(0);
        Ok(InsertProps {
            connect: SQLImplementation::SQLite(self.clone()),
            grid,
            table: table.to_string(),
            header,
            create: false,
        })
    }

    fn create(&self) -> CreateProps {
        CreateProps {
            connect: SQLImplementation::SQLite(self.clone()),
        }
    }

    fn alter(&self) -> AlterProps {
        AlterProps {
            connect: SQLImplementation::SQLite(self.clone()),
        }
    }

    fn delete(&self, table: &str) -> DeleteProps {
        let table = Table::new(table);
        DeleteProps {
            connect: SQLImplementation::SQLite(self.clone()),
            table,
            clause: None,
        }
    }
}
