use crate::{
    Error, QueryBuilder, SQLImplementation,
    data_types::ToSQLData,
    sql_implementations::OracleConnect,
    statements::{
        alter::AlterProps,
        create::CreateProps,
        delete::DeleteProps,
        insert::InsertProps,
        select::{Column, SelectProps},
        update::UpdateProps,
    },
};

impl OracleConnect {
    #[inline]
    pub fn new(connection_string: &str, username: &str, password: &str) -> Result<Self, Error> {
        match oracle::Connection::connect(&username, &password, &connection_string) {
            Ok(_) => Ok(Self {
                connection_string: connection_string.to_string(),
                username: username.to_string(),
                password: password.to_string(),
            }),
            Err(e) => Err(Error::OracleError(e)),
        }
    }
}

impl QueryBuilder for OracleConnect {
    fn select(&self, table: &str, columns: Vec<Column>) -> SelectProps {
        SQLImplementation::Oracle(self.clone()).select_initialization(table, columns)
    }

    fn update(&self, table: &str) -> UpdateProps {
        SQLImplementation::Oracle(self.clone()).update_initialization(table)
    }

    fn insert<T: ToSQLData>(&self, table: &str, data: Vec<Vec<T>>) -> Result<InsertProps, Error> {
        SQLImplementation::Oracle(self.clone()).insert_initialization(table, data)
    }

    fn create(&self) -> CreateProps {
        SQLImplementation::Oracle(self.clone()).create_initialization()
    }

    fn alter(&self) -> AlterProps {
        SQLImplementation::Oracle(self.clone()).alter_initialization()
    }

    fn delete(&self, table: &str) -> DeleteProps {
        SQLImplementation::Oracle(self.clone()).delete_initialization(table)
    }
}
