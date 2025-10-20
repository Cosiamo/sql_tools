use crate::{
    data_types::{SQLDataTypes, ToSQLData}, statements::{
        alter::AlterProps,
        create::CreateProps,
        delete::DeleteProps,
        insert::InsertProps,
        select::{Column, Limit, OrderBy, SelectBuilder, SelectProps},
        update::UpdateInitialization,
    }, utils::remove_invalid_chars, Error, QueryBuilder, SQLImplementation
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
        let table = table.trim();

        let mut header = vec![];
        for col in columns {
            if col.contains(".") {
                let col_props = col.split(".").collect::<Vec<&str>>();
                header.push(
                    Column { name: col_props[col_props.len() - 1].to_string(), table: col_props[0].to_string() }
                );
            } else {
                header.push(
                    Column { name: col.to_string(), table: table.to_string() }
                );
            }
        }

        SelectProps {
            connect: SQLImplementation::SQLite(self.clone()),
            columns: header,
            table: table.to_string(),
            joins: vec![],
            clause: None,
            order_by: (None, OrderBy::None),
            group_by: None,
            limit: Limit {
                limit: None,
                offset: None,
            },
            return_header: false,
        }
    }

    fn update(&self, table: &str) -> UpdateInitialization {
        UpdateInitialization {
            connect: SQLImplementation::SQLite(self.clone()),
            table: table.to_owned(),
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
                    .map(|cell| cell.fmt_data())
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
        let table = table.to_string();
        DeleteProps {
            connect: SQLImplementation::SQLite(self.clone()),
            table,
            clause: None,
        }
    }
}
