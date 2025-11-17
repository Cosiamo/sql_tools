use crate::data_types::SQLDataTypes;

pub mod implement;
pub(crate) mod utils;

#[derive(Debug)]
/// The argument type for the `where_in`, `where_not`, `and`, `and_not`, `or`, and `or_not` methods.
/// This is split up specifically to prevent SQL injections and to be more intentional with building query structures.
pub enum WhereArg {
    Values(Vec<SQLDataTypes>),
    Like(String),
    Query(String),
    NULL,
}

/// Trait to add a WHERE clause to a SQL statement.
pub trait QueryConjunctions {
    /// Adds a WHERE clause to your query.
    /// ```no_run
    /// let values = WhereArg::Values(vec![
    ///    SQLDataTypes::Varchar("Q2".to_string()),  SQLDataTypes::Varchar("Q3".to_string())
    /// ]);
    /// let data = conn.select("quarterly_earnings", vec!["revenue", "profit"])
    ///     .where_in("quarter", values)
    ///     .build()?;
    /// ```
    ///
    /// ```sql
    /// SELECT revenue, profit FROM quarterly_earning WHERE quarter IN ('Q2', 'Q3');
    /// ```
    ///
    /// ```no_run
    /// let like = WhereArg::Like("Bob%".to_string());
    /// let data = conn.delete("employees")
    ///     .where_in("name", like)
    ///     .build()?;
    /// ```
    ///
    /// ```sql
    /// DELETE FROM employees WHERE name LIKE 'Bob%';
    /// ```
    ///
    /// ```no_run
    /// let query = WhereArg::Query("SELECT name FROM customers");
    /// let data = conn.select("employees", vec!["name", "position"])
    ///     .where_in("name", query)
    ///     .build()?;
    /// ```
    ///
    /// ```sql
    /// SELECT name, position FROM employees WHERE name in (SELECT name FROM customers);
    /// ```
    ///
    /// ```no_run
    /// let null = WhereArg::Null;
    /// let data = conn.select("employees", vec!["name", "position"])
    ///     .where_in("hours_worked", null)
    ///     .build()?;
    /// ```
    ///
    /// ```sql
    /// SELECT name, position FROM employees WHERE hours_worked IS NULL;
    /// ```
    fn where_in(self, column: &str, values: WhereArg) -> Self;

    /// Adds a WHERE NOT clause to your query.
    /// ```no_run
    /// let values = WhereArg::Values(vec![
    ///    SQLDataTypes::Varchar("Q1".to_string()),  SQLDataTypes::Varchar("Q4".to_string())
    /// ]);
    /// let data = conn.select("quarterly_earnings", vec!["revenue", "profit"])
    ///     .where_not("quarter", values)
    ///     .build()?;
    /// ```
    ///
    /// ```sql
    /// SELECT revenue, profit FROM quarterly_earning WHERE quarter NOT IN ('Q2', 'Q3');
    /// ```
    ///
    /// ```no_run
    /// let like = WhereArg::Like("Bob%".to_string());
    /// let data = conn.select("employees", vec!["name", "position"])
    ///     .where_not("name", like)
    ///     .build()?;
    /// ```
    ///
    /// ```sql
    /// DELETE FROM employees WHERE name NOT LIKE 'Bob%';
    /// ```
    ///
    /// ```no_run
    /// let query = WhereArg::Query(
    ///     "SELECT name FROM customers"
    /// );
    /// let data = conn.select("employees", vec!["name", "position"])
    ///     .where_not("name", query)
    ///     .build()?;
    /// ```
    ///
    /// ```sql
    /// SELECT name, position FROM employees WHERE name NOT IN (SELECT name FROM customers);
    /// ```
    ///
    /// ```no_run
    /// conn.update("employees")
    ///     .set("hours_worked", 0)
    ///     .where_not("hours_worked", WhereArg::NULL)
    ///     .build()?;
    /// ```
    /// Is the same as:
    /// ```sql
    /// UPDATE employees
    /// SET hours_worked = 0
    /// WHERE hours_worked IS NOT NULL;
    fn where_not(self, column: &str, values: WhereArg) -> Self;

    /// Adds an 'AND' to a WHERE clause.
    /// ```no_run
    /// let timezones = WhereArg::Values(vec![
    ///     SQLDataTypes::Varchar("Central European Standard Time"), SQLDataTypes::Varchar("Eastern European Standard Time")
    /// ]);
    /// let europe = WhereArg::Values(vec![SQLDataTypes::Varchar("Europe")]);
    /// let data: Vec<Vec<SQLDataTypes>> = conn
    ///     .select("users", vec!["first_name", "email"])
    ///     .where_in("continent", europe)
    ///     .and("timezone", timezones)
    ///     .build()?;
    /// ```
    ///
    /// ```sql
    /// SELECT first_name, email FROM users
    /// WHERE continent IN ('Europe')
    /// AND timezone IN ('Central European Standard Time', 'Eastern European Standard Time');
    /// ```
    ///
    /// AND LIKE
    /// ```no_run
    /// let like = WhereArg::Like("Nor%".to_string());
    /// let europe = WhereArg::Values(vec![SQLDataTypes::Varchar("Europe")]);
    /// let data: Vec<Vec<SQLDataTypes>> = conn
    ///     .select("users", vec!["first_name", "email"])
    ///     .where_in("continent", europe)
    ///     .and("country", like)
    ///     .build()?;
    /// ```
    ///
    /// ```sql
    /// SELECT first_name, email FROM users
    /// WHERE continent IN ('Europe')
    /// AND country LIKE 'Nor%';
    /// ```
    ///
    /// AND IN (query)
    /// ```no_run
    /// let query = WhereArg::Query("SELECT email FROM ban_list".to_string());
    /// let europe = WhereArg::Values(vec![SQLDataTypes::Varchar("Europe")]);
    /// let data: Vec<Vec<SQLDataTypes>> = conn
    ///     .select("users", vec!["first_name", "email"])
    ///     .where_in("continent", europe)
    ///     .and("email", query)
    ///     .build()?;
    /// ```
    ///
    /// ```sql
    /// SELECT first_name, email FROM users
    /// WHERE continent IN ('Europe')
    /// AND email IN ('SELECT email FROM ban_list');
    /// ```
    ///
    /// AND IS NULL
    /// ```no_run
    /// let europe = WhereArg::Values(vec![SQLDataTypes::Varchar("Europe")]);
    /// let data: Vec<Vec<SQLDataTypes>> = conn
    ///     .select("users", vec!["first_name", "email"])
    ///     .where_in("continent", europe)
    ///     .and("phone_number", WhereArg::NULL)
    ///     .build()?;
    /// ```
    ///
    /// ```sql
    /// SELECT first_name, email FROM users
    /// WHERE continent IN ('Europe')
    /// AND phone_number IS NULL;
    /// ```
    fn and(self, column: &str, values: WhereArg) -> Self;

    /// Adds a 'OR' to a WHERE clause.
    /// ```no_run
    /// let values = WhereArg::Values(vec!["United States", "Brazil"]);
    /// let europe = WhereArg::Values(vec![SQLDataTypes::Varchar("Europe")]);
    /// let data: Vec<Vec<SQLDataTypes>> = conn
    ///     .select("users", vec!["first_name", "email"])
    ///     .where_in("continent", europe)
    ///     .or("country", values)
    ///     .build()?;
    /// ```
    ///
    /// ```sql
    /// SELECT first_name, email FROM users
    /// WHERE continent IN ('Europe')
    /// OR country IN ('United States', 'Brazil');
    /// ```
    ///
    /// OR LIKE
    /// ```no_run
    /// let like = WhereArg::Like("United%".to_string());
    /// let europe = WhereArg::Values(vec![SQLDataTypes::Varchar("Europe")]);
    /// let data: Vec<Vec<SQLDataTypes>> = conn
    ///     .select("users", vec!["first_name", "email"])
    ///     .where_in("continent", europe)
    ///     .or("country", like)
    ///     .build()?;
    /// ```
    ///
    /// ```sql
    /// SELECT first_name, email FROM users
    /// WHERE continent IN ('Europe')
    /// OR country LIKE 'United%';
    /// ```
    ///
    /// OR IN (query)
    /// ```no_run
    /// let query = WhereArg::Query("SELECT country FROM countries WHERE population > 100000000".to_string());
    /// let europe = WhereArg::Values(vec![SQLDataTypes::Varchar("Europe")]);
    /// let data: Vec<Vec<SQLDataTypes>> = conn
    ///     .select("users", vec!["first_name", "email"])
    ///     .where_in("continent", europe)
    ///     .or("country", query)
    ///     .build()?;
    /// ```
    ///
    /// ```sql
    /// SELECT first_name, email FROM users
    /// WHERE continent IN ('Europe')
    /// OR country IN (SELECT country FROM countries WHERE population > 10000000);
    /// ```
    ///
    /// OR IS NULL
    /// ```no_run
    /// let europe = WhereArg::Values(vec![SQLDataTypes::Varchar("Europe")]);
    /// let data: Vec<Vec<SQLDataTypes>> = conn
    ///     .select("users", vec!["first_name", "email"])
    ///     .where_in("continent", europe)
    ///     .or("country", WhereArg::NULL)
    ///     .build()?;
    /// ```
    ///
    /// ```sql
    /// SELECT first_name, email FROM users
    /// WHERE continent IN ('Europe')
    /// OR country IS NULL;
    /// ```
    fn or(self, column: &str, values: WhereArg) -> Self;

    /// Adds an 'AND NOT' to a WHERE clause.
    /// ```no_run
    /// let values = WhereArg::Values(vec!["France", "Spain"]);
    /// let europe = WhereArg::Values(vec![SQLDataTypes::Varchar("Europe")]);
    /// let data: Vec<Vec<SQLDataTypes>> = conn
    ///     .select("users", vec!["first_name", "email"])
    ///     .where_in("continent", europe)
    ///     .and_not("country", values)
    ///     .build()?;
    /// ```
    ///
    /// ```sql
    /// SELECT first_name, email FROM users
    /// WHERE continent IN ('Europe')
    /// AND country NOT IN ('France', 'Spain');
    /// ```
    ///
    /// AND NOT LIKE
    /// ```no_run
    /// let like = WhereArg::Like("Nor%".to_string());
    /// let europe = WhereArg::Values(vec![SQLDataTypes::Varchar("Europe")]);
    /// let data: Vec<Vec<SQLDataTypes>> = conn
    ///     .select("users", vec!["first_name", "email"])
    ///     .where_in("continent", europe)
    ///     .and_not("country", like)
    ///     .build()?;
    /// ```
    ///
    /// ```sql
    /// SELECT first_name, email FROM users
    /// WHERE continent IN ('Europe')
    /// AND country NOT LIKE 'Nor%';
    /// ```
    ///
    /// AND NOT IN (query)
    /// ```no_run
    /// let query = WhereArg::Query("SELECT email FROM ban_list".to_string());
    /// let europe = WhereArg::Values(vec![SQLDataTypes::Varchar("Europe")]);
    /// let data: Vec<Vec<SQLDataTypes>> = conn
    ///     .select("users", vec!["first_name", "email"])
    ///     .where_in("continent", europe)
    ///     .and_not("email", query)
    ///     .build()?;
    /// ```
    ///
    /// ```sql
    /// SELECT first_name, email FROM users
    /// WHERE continent IN ('Europe')
    /// AND email NOT IN ('SELECT email FROM ban_list');
    /// ```
    ///
    /// AND IS NOT NULL
    /// ```no_run
    /// let europe = WhereArg::Values(vec![SQLDataTypes::Varchar("Europe")]);
    /// let data: Vec<Vec<SQLDataTypes>> = conn
    ///     .select("users", vec!["first_name", "email"])
    ///     .where_in("continent", europe)
    ///     .and_not("email", WhereArg::NULL)
    ///     .build()?;
    /// ```
    ///
    /// ```sql
    /// SELECT first_name, email FROM users
    /// WHERE continent IN ('Europe')
    /// AND email IS NOT NULL;
    /// ```
    fn and_not(self, column: &str, values: WhereArg) -> Self;

    /// Adds a 'OR NOT' to a WHERE clause.
    /// ```no_run
    /// let values = WhereArg::Values(vec!["United States", "Brazil"]);
    /// let europe = WhereArg::Values(vec![SQLDataTypes::Varchar("Europe")]);
    /// let data: Vec<Vec<SQLDataTypes>> = conn
    ///     .select("users", vec!["first_name", "email"])
    ///     .where_not("continent", europe)
    ///     .or_not("country", values)
    ///     .build()?;
    /// ```
    ///
    /// ```sql
    /// SELECT first_name, email FROM users
    /// WHERE continent NOT IN ('Europe')
    /// OR country NOT IN ('United States', 'Brazil');
    /// ```
    ///
    /// OR NOT LIKE
    /// ```no_run
    /// let like = WhereArg::Like("United%".to_string());
    /// let europe = WhereArg::Values(vec![SQLDataTypes::Varchar("Europe")]);
    /// let data: Vec<Vec<SQLDataTypes>> = conn
    ///     .select("users", vec!["first_name", "email"])
    ///     .where_in("continent", europe)
    ///     .or_not("country", like)
    ///     .build()?;
    /// ```
    ///
    /// ```sql
    /// SELECT first_name, email FROM users
    /// WHERE continent IN ('Europe')
    /// OR country NOT LIKE 'United%';
    /// ```
    ///
    /// OR NOT IN (query)
    /// ```no_run
    /// let query = WhereArg::Query("SELECT country FROM countries WHERE population > 100000000".to_string());
    /// let europe = WhereArg::Values(vec![SQLDataTypes::Varchar("Europe")]);
    /// let data: Vec<Vec<SQLDataTypes>> = conn
    ///     .select("users", vec!["first_name", "email"])
    ///     .where_not("continent", europe)
    ///     .or_not("country", query)
    ///     .build()?;
    /// ```
    ///
    /// ```sql
    /// SELECT first_name, email FROM users
    /// WHERE NOT continent IN ('Europe')
    /// OR country NOT IN (SELECT country FROM countries WHERE population > 10000000);
    /// ```
    ///
    /// OR NOT NULL
    /// ```no_run
    /// let europe = WhereArg::Values(vec![SQLDataTypes::Varchar("Europe")]);
    /// let data: Vec<Vec<SQLDataTypes>> = conn
    ///     .select("users", vec!["first_name", "email"])
    ///     .where_in("continent", europe)
    ///     .or_not("email", WhereArg::NULL)
    ///     .build()?;
    /// ```
    ///
    /// ```sql
    /// SELECT first_name, email FROM users
    /// WHERE continent IN ('Europe')
    /// OR email IS NOT NULL;
    /// ```
    fn or_not(self, column: &str, values: WhereArg) -> Self;
}
