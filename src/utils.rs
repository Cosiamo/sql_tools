use rand::Rng;
use std::iter;

use crate::{
    Error, QueryBuilder, sql_variations::OracleConnect, statements::select::SelectBuilder,
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

pub(crate) fn generate_id(len: usize) -> String {
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let mut rng = rand::rng();
    let one_char = || CHARSET[rng.random_range(0..CHARSET.len())] as char;
    iter::repeat_with(one_char).take(len).collect()
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
