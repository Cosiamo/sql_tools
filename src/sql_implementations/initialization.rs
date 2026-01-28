use crate::{
    Error, SQLImplementation,
    data_types::{SQLDataTypes, ToSQLData},
    statements::{
        alter::AlterProps,
        create::CreateProps,
        delete::DeleteProps,
        insert::InsertProps,
        select::{Column, Limit, SelectProps},
        update::UpdateProps,
    },
    utils::remove_invalid_chars,
};

impl SQLImplementation {
    pub(crate) fn select_initialization(self, table: &str, columns: Vec<Column>) -> SelectProps {
        let table = table.trim();
        SelectProps {
            connect: self,
            columns,
            table: table.to_string(),
            joins: vec![],
            clause: None,
            order_by: None,
            group_by: None,
            limit: Limit {
                limit: None,
                offset: None,
            },
            return_header: false,
        }
    }

    pub(crate) fn update_initialization(self, table: &str) -> UpdateProps {
        UpdateProps {
            connect: self,
            table: table.to_owned(),
            set_match: Vec::new(),
            clause: None,
        }
    }

    pub(crate) fn insert_initialization<T: ToSQLData>(
        self,
        table: &str,
        data: Vec<Vec<T>>,
    ) -> Result<InsertProps, Error> {
        if data.len() < 2 {
            return Err(Error::NoHeading);
        }
        let mut grid = data
            .iter()
            .map(|row| {
                row.iter()
                    .map(|cell| cell.to_sql_fmt())
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
            connect: self,
            grid,
            table: table.to_string(),
            header,
            create: false,
        })
    }

    pub(crate) fn create_initialization(self) -> CreateProps {
        CreateProps { connect: self }
    }

    pub(crate) fn alter_initialization(self) -> AlterProps {
        AlterProps { connect: self }
    }

    pub(crate) fn delete_initialization(self, table: &str) -> DeleteProps {
        let table = table.to_string();
        DeleteProps {
            connect: self,
            table,
            clause: None,
        }
    }
}
