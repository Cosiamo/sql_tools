use crate::{
    statements::{alter::sql_implementations::{oracle::alter_oracle, sqlite::alter_sqlite}, create::CreateDataTypes}, Error, SQLVariation
};

use super::{AlterBuilder, AlterColumns, AlterProps, AlterTable, AlterTableBuilder, Altered};

impl AlterBuilder for AlterProps {
    // fn session(self, schema: &str, value: &str) -> Result<(), Error> {
    //     let query = format!("ALTER SESSION SET {schema} = '{value}'");
    //     match self.connect {
    //         SQLVariation::Oracle(oracle_connect) => alter(oracle_connect, query),
    //     }
    // }

    fn table(self, table_name: &str) -> AlterTable {
        let query = format!("ALTER TABLE {table_name} ");
        AlterTable {
            connect: self.connect,
            query,
            table_name: table_name.to_string(),
        }
    }
}

impl AlterTableBuilder for AlterTable {
    fn add(mut self, columns: Vec<AlterColumns>) -> Altered {
        let cols = columns
            .iter()
            .map(|cols| alter_cols_fmt(cols))
            .collect::<Vec<String>>();
        let add = format!("ADD ({})", cols.join(", "));
        self.query = format!("{} {add}", &self.query);
        Altered {
            connect: self.connect,
            query: self.query,
        }
    }

    fn modify(mut self, columns: Vec<AlterColumns>) -> Altered {
        let cols = columns
            .iter()
            .map(|cols| alter_cols_fmt(cols))
            .collect::<Vec<String>>();
        let modify = format!("MODIFY ({})", cols.join(", "));
        self.query = format!("{} {modify}", &self.query);
        Altered {
            connect: self.connect,
            query: self.query,
        }
    }

    fn drop(mut self, column: &str) -> Altered {
        let drop = format!("DROP COLUMN {column}");
        self.query = format!("{} {drop}", &self.query);
        Altered {
            connect: self.connect,
            query: self.query,
        }
    }

    fn rename_column(mut self, column: &str, new_name: &str) -> Altered {
        let rename = format!("RENAME COLUMN {column} TO {new_name}");
        self.query = format!("{} {rename}", &self.query);
        Altered {
            connect: self.connect,
            query: self.query,
        }
    }

    fn rename(mut self, new_table_name: &str) -> Altered {
        let rename = format!("RENAME TO {new_table_name}");
        self.query = format!("{} {rename}", &self.query);
        Altered {
            connect: self.connect,
            query: self.query,
        }
    }
}

pub fn alter_cols_fmt(cols: &AlterColumns) -> String {
    let data_type = match cols.data_type {
        CreateDataTypes::VARCHAR(size) => format!("VARCHAR2 ({size})"),
        CreateDataTypes::NUMBER => format!("NUMBER"),
        CreateDataTypes::FLOAT => format!("FLOAT"),
        CreateDataTypes::DATE => format!("DATE"),
    };
    let mut res = format!("{} {data_type}", cols.name);
    res = if let Some(sql) = &cols.default {
        format!("{res} DEFAULT '{sql}'")
    } else {
        res
    };
    if let true = cols.not_null {
        format!("{res} NOT NULL")
    } else {
        res
    }
}

impl Altered {
    /// Builds the `ALTER` query.
    pub fn build(self) -> Result<(), Error> {
        match self.connect {
            SQLVariation::Oracle(conn) => alter_oracle(conn, self.query),
            SQLVariation::SQLite(conn) => alter_sqlite(conn, self.query),
        }
    }
}
