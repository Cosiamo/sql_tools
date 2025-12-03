use crate::{
    Error, QueryBuilder,
    query_conjunctions::{QueryConjunctions, WhereArg},
    sql_implementations::OracleConnect,
    statements::select::{Column, ColumnProps, SelectBuilder},
};

pub(crate) fn remove_invalid_chars(input: &String) -> String {
    input
        .trim()
        .replace(|c: char| !c.is_ascii(), "")
        .replace(" ", "_")
        .replace("-", "_")
        .replace("'", "")
        .replace("%", "")
        .replace("!", "")
        .replace("?", "")
        .replace("|", "")
        .replace("#", "")
        .replace("\\", "")
        .replace("/", "")
        .replace("(", "")
        .replace(")", "")
        .replace("+", "")
        .replace("#", "")
}

impl OracleConnect {
    pub fn does_table_exist(&self, table: &str) -> Result<bool, Error> {
        let value = WhereArg::Values(vec![crate::data_types::SQLDataTypes::Varchar(
            table.to_ascii_uppercase(),
        )]);
        let column = Column::Name(ColumnProps {
            name: "table_name".to_string(),
            table: "user_tables".to_string(),
        });
        let exists = self
            .select("user_tables", vec![column])
            .where_in(
                &ColumnProps {
                    name: "upper(table_name)".to_string(),
                    table: "user_table".to_string(),
                },
                value,
            )
            .build_single_thread()?;
        if exists.len() > 0 {
            Ok(true)
        } else {
            Ok(false)
        }
    }

    pub fn get_table_names(&self) -> Result<Vec<String>, Error> {
        let column = Column::Name(ColumnProps {
            name: "table_name".to_string(),
            table: "user_tables".to_string(),
        });
        let tables = self
            .select("user_tables", vec![column])
            .build_single_thread()?;
        let names = tables
            .iter()
            .map(|row| row[0].to_string())
            .collect::<Vec<String>>();
        Ok(names)
    }
}
