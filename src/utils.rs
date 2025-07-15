use crate::{Error, QueryBuilder, clauses::select::SelectBuilder, variations::OracleConnect};

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
        let exists = self
            .select("user_tables", vec!["table_name"])
            .where_in("upper(table_name)", vec![table.to_ascii_uppercase()])
            .build_single_thread()?;
        if exists.len() > 0 {
            Ok(true)
        } else {
            Ok(false)
        }
    }

    pub fn get_table_names(&self) -> Result<Vec<String>, Error> {
        let tables = self
            .select("user_tables", vec!["table_name"])
            .build_single_thread()?;
        let names = tables
            .iter()
            .map(|row| row[0].to_string())
            .collect::<Vec<String>>();
        Ok(names)
    }
}
