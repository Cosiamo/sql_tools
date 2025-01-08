use crate::{create::CreateDataTypes, SQLVariation};

pub mod oracle_sql;
pub mod implement;

#[derive(Debug)]
pub struct AlterProps {
    pub connect: SQLVariation,
}

#[derive(Debug)]
pub struct AlterTable {
    pub connect: SQLVariation,
    pub query: String,
    pub table_name: String,
}

#[derive(Debug)]
pub struct AlterColumns {
    pub name: String,
    pub data_type: CreateDataTypes,
    pub default: Option<String>,
    pub not_null: bool,
}

#[derive(Debug)]
pub struct Altered {
    pub connect: SQLVariation,
    pub query: String,
}

pub trait AlterBuilder {
    // ===== Might not be able to use this unless there's an overhaul to how connections work =====
    // fn session(self, schema: &str, value: &str) -> Result<(), Error>;

    /// Alters a table by either [adding](`AlterTable::add`), [modifying](`AlterTable::modify`), [dropping](`AlterTable::drop`), or [renaming](`AlterTable::rename_column`) a column.
    /// You can also [rename the table](`AlterTable::rename`).
    fn table(self, table_name: &str) -> AlterTable;
}

pub trait AlterTableBuilder {
    /// Adds a column(s) to a table.
    /// 
    /// ```no_run
    /// let conn = OracleConnect::new(connection_string, username, password)?;
    /// let column = AlterColumns {
    ///     name: "title".to_string(),
    ///     data_type: CreateDataTypes::VARCHAR(10),
    ///     default: Some("PMO".to_string()),
    ///     not_null: true,
    /// };
    /// conn.alter()
    ///     .table("employees")
    ///     .add(vec![column])
    ///     .build()?;
    /// ```
    /// 
    /// The same as:
    /// ```sql
    /// ALTER TABLE employees ADD title VARCHAR2(10) DEFAULT 'PMO' NOT NULL;
    /// ```
    fn add(self, columns: Vec<AlterColumns>) -> Altered;

    /// Modifies a column(s) on a table.
    /// 
    /// ```no_run
    /// let conn = OracleConnect::new(connection_string, username, password)?;
    /// let column = AlterColumns {
    ///     name: "title".to_string(),
    ///     data_type: CreateDataTypes::VARCHAR(10),
    ///     default: Some("PMO".to_string()),
    ///     not_null: true,
    /// };
    /// conn.alter()
    ///     .table("employees")
    ///     .modify(vec![column])
    ///     .build()?;
    /// ```
    /// 
    /// The same as:
    /// ```sql
    /// ALTER TABLE employees MODIFY title VARCHAR2(10) DEFAULT 'PMO' NOT NULL;
    /// ```
    fn modify(self, columns: Vec<AlterColumns>) -> Altered;

    /// Drops a column from a table.
    /// 
    /// ```no_run
    /// let conn = OracleConnect::new(connection_string, username, password)?;
    /// conn.alter()
    ///     .table("sales")
    ///     .drop("description")
    ///     .build()?;
    /// ```
    /// 
    /// The same as:
    /// ```sql
    /// ALTER TABLE sales DROP COLUMN description;
    /// ```
    fn drop(self, column: &str) -> Altered;

    /// Renames a column from a table.
    /// 
    /// ```no_run
    /// let conn = OracleConnect::new(connection_string, username, password)?;
    /// conn.alter()
    ///     .table("sales")
    ///     .rename("salesman", "employee")
    ///     .build()?;
    /// ```
    /// 
    /// The same as:
    /// ```sql
    /// ALTER TABLE sales RENAME COLUMN salesman TO employee;
    /// ```
    fn rename_column(self, column: &str, new_name: &str) -> Altered;

    /// Renames a table.
    /// 
    /// ```no_run
    /// let conn = OracleConnect::new(connection_string, username, password)?;
    /// conn.alter()
    ///     .table("local_sales")
    ///     .rename("regional_sales")
    ///     .build()?;
    /// ```
    /// 
    /// The same as:
    /// ```sql
    /// ALTER TABLE local_sales RENAME TO regional_sales;
    /// ```
    fn rename(self, new_table_name: &str) -> Altered;
}