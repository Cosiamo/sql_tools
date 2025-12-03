use crate::{
    Error, QueryBuilder, SQLImplementation,
    data_types::{SQLDataTypes, ToSQLData},
    statements::{
        alter::AlterProps,
        create::CreateProps,
        delete::DeleteProps,
        insert::InsertProps,
        select::{Column, SelectProps},
        update::UpdateInitialization,
    },
};

use super::SQLiteConnect;

impl SQLiteConnect {
    /// Opens new SQLite connection based of the path of the database file.
    #[inline]
    pub fn from_path(path: &str) -> Self {
        SQLiteConnect::Path(path.to_string())
    }

    /// Opens new SQLite connection in memory. This database ceases to exist once the connection is closed.
    #[inline]
    pub fn in_memory() -> Self {
        SQLiteConnect::Memory
    }

    pub(crate) fn initialize_connection(&self) -> Result<rusqlite::Connection, Error> {
        match self {
            SQLiteConnect::Path(path) => match rusqlite::Connection::open(path.to_string()) {
                Ok(val) => return Ok(val),
                Err(err) => return Err(Error::SQLiteError(err)),
            },
            SQLiteConnect::Memory => match rusqlite::Connection::open_in_memory() {
                Ok(val) => return Ok(val),
                Err(err) => return Err(Error::SQLiteError(err)),
            },
        }
    }

    pub fn table_info(&self, table: &str) -> Result<Vec<String>, Error> {
        if let SQLiteConnect::Path(path) = self {
            if path == "" {
                return Err(Error::TableDoesNotExist);
            };
        }

        let conn = self.initialize_connection()?;

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
            .map(|row| row.to_string())
            .collect::<Vec<String>>();
        Ok(res)
    }
}

impl QueryBuilder for SQLiteConnect {
    fn select(&self, table: &str, columns: Vec<&Column>) -> SelectProps {
        SQLImplementation::SQLite(self.clone()).select_initialization(table, columns)
    }

    fn update(&self, table: &str) -> UpdateInitialization {
        SQLImplementation::SQLite(self.clone()).update_initialization(table)
    }

    fn insert<T: ToSQLData>(&self, table: &str, data: Vec<Vec<T>>) -> Result<InsertProps, Error> {
        SQLImplementation::SQLite(self.clone()).insert_initialization(table, data)
    }

    fn create(&self) -> CreateProps {
        SQLImplementation::SQLite(self.clone()).create_initialization()
    }

    fn alter(&self) -> AlterProps {
        SQLImplementation::SQLite(self.clone()).alter_initialization()
    }

    fn delete(&self, table: &str) -> DeleteProps {
        SQLImplementation::SQLite(self.clone()).delete_initialization(table)
    }
}
