use crate::{
    data_types::{SQLDataTypes, ToSQLData}, statements::{
        alter::AlterProps,
        create::CreateProps,
        delete::DeleteProps,
        insert::InsertProps,
        select::SelectProps,
        update::UpdateInitialization,
    }, Error, QueryBuilder, SQLImplementation
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
        let conn = rusqlite::Connection::open(&connection.path.clone())?;

        let query = format!("select name from PRAGMA_TABLE_INFO('{table}')");
        let mut stmt = conn.prepare(&query)?;
        let mut rows = stmt.query([])?;
        let mut columns = Vec::new();
        while let Some(row) = rows.next()? {
            let p = Box::new(row.get::<usize, SQLDataTypes>(0)?);
            columns.push(p);
        }

        if columns.len() == 0 {
            return Err(Error::TableDoesNotExist);
        };

        let res = columns
            .iter()
            .map(|row| {
                row.to_string()
            })
            .collect::<Vec<String>>();
        Ok(res)
    }
}

impl QueryBuilder for SQLiteConnect {
    fn select(&self, table: &str, columns: Vec<&str>) -> SelectProps {
        SQLImplementation::SQLite(self.clone())
            .select_initialization(table, columns)
    }

    fn update(&self, table: &str) -> UpdateInitialization {
        SQLImplementation::SQLite(self.clone())
            .update_initialization(table)
    }

    fn insert<T: ToSQLData>(&self, table: &str, data: Vec<Vec<T>>) -> Result<InsertProps, Error> {
        SQLImplementation::SQLite(self.clone())
            .insert_initialization(table, data)
    }

    fn create(&self) -> CreateProps {
        SQLImplementation::SQLite(self.clone())
            .create_initialization()
    }

    fn alter(&self) -> AlterProps {
        SQLImplementation::SQLite(self.clone())
            .alter_initialization()
    }

    fn delete(&self, table: &str) -> DeleteProps {
        SQLImplementation::SQLite(self.clone())
            .delete_initialization(table)
    }
}
