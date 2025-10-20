use crate::{
    data_types::{SQLDataTypes, ToSQLData}, sql_implementations::OracleConnect, statements::{
        alter::AlterProps,
        create::CreateProps,
        delete::DeleteProps,
        insert::InsertProps,
        select::{Column, Limit, OrderBy, SelectProps},
        update::UpdateInitialization,
    }, utils::remove_invalid_chars, Error, QueryBuilder, SQLImplementation, Table
};

impl OracleConnect {
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
            connect: SQLImplementation::Oracle(self.clone()),
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

    fn update(&self, table: &Table) -> UpdateInitialization {
        UpdateInitialization {
            connect: SQLImplementation::Oracle(self.clone()),
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
            connect: SQLImplementation::Oracle(self.clone()),
            grid,
            table: table.to_string(),
            header,
            create: false,
        })
    }

    fn create(&self) -> CreateProps {
        CreateProps {
            connect: SQLImplementation::Oracle(self.clone()),
        }
    }

    fn alter(&self) -> AlterProps {
        AlterProps {
            connect: SQLImplementation::Oracle(self.clone()),
        }
    }

    fn delete(&self, table: &str) -> DeleteProps {
        let table = Table::new(table);
        DeleteProps {
            connect: SQLImplementation::Oracle(self.clone()),
            table,
            clause: None,
        }
    }
}
