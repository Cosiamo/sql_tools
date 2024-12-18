use crate::{create::CreateProps, data_types::{ToSQLData, SQLDataTypes}, insert::InsertProps, select::SelectProps, sql_variations::OracleConnect, update::UpdateProps, utils::remove_invalid_chars, QueryBuilder, SQLVariation};

impl OracleConnect {
    pub fn new(connection_string: &str, username: &str, password: &str) -> Self {
        Self {
            connection_string: connection_string.to_string(),
            username: username.to_string(),
            password: password.to_string(),
        }
    }
}

impl QueryBuilder for OracleConnect {
    fn select(&self, table: &str, columns: Vec<&str>) -> SelectProps {
        let fmt_cols = columns.iter().map(|cols| {
            remove_invalid_chars(&cols.to_string())
        }).collect::<Vec<String>>();
        SelectProps {
            connect: SQLVariation::Oracle(self.clone()),
            columns: fmt_cols,
            table: table.to_string(),
            clause: None,
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