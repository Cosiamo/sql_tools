use crate::{create::CreateProps, data_types::{SQLDataTypes, ToSQLData}, insert::InsertProps, select::{OrderBy, SelectProps}, sql_variations::OracleConnect, update::UpdateProps, utils::remove_invalid_chars, Error, QueryBuilder, SQLVariation};

impl OracleConnect {
    pub fn new(connection_string: &str, username: &str, password: &str) -> Result<Self, crate::sql_variations::oracle_sql::Error> {
        match oracle::Connection::connect(&username, &password, &connection_string) {
            Ok(_) => Ok(
                Self {
                    connection_string: connection_string.to_string(),
                    username: username.to_string(),
                    password: password.to_string(),
                }
            ),
            Err(e) => Err(Error::OracleError(e)),
        }
    }
}

impl QueryBuilder for OracleConnect {
    fn select(&self, table: &str, columns: Vec<&str>) -> SelectProps {
        let fmt_cols = columns.iter().map(|cols| { cols.to_string() }).collect::<Vec<String>>();
        SelectProps {
            connect: SQLVariation::Oracle(self.clone()),
            columns: fmt_cols,
            table: table.to_string(),
            clause: None,
            order_by: (None, OrderBy::None)
        }
    }
    
    fn update(&self, table: &str) -> UpdateProps {
        UpdateProps {
            connect: SQLVariation::Oracle(self.clone()),
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
            connect: SQLVariation::Oracle(self.clone()),
            grid,
            table: table.to_string(),
            header,
        }
    }
    
    fn create(&self) -> CreateProps {
        CreateProps {
            connect: SQLVariation::Oracle(self.clone()),
        }
    }
}