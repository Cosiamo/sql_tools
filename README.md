# SQL Tools
A rust crate meant to make SQL queries simple and communication between various SQL versions easy. The goal is to have most major variations of SQL compatible, however only Oracle SQL and SQLite are available right now. This is an evolution of [oracle_sql_tools](https://crates.io/crates/oracle_sql_tools). This project is constantly changing, check out the [change logs](https://github.com/Cosiamo/sql_tools/blob/main/CHANGELOG.md) to see what's new.

# How To Use
In your `cargo.toml` file:
```toml
[dependencies]
sql_tools = "<CURRENT_VERSION>"
# chrono is required if you're working with dates 
chrono = "0.4.41" 
```

To start using SQL Tools, you need a new connection.

### Oracle SQL
```rust
use sql_tools::sql_variations::OracleConnect;

let username = "new_user";
let password = "password123!";
let connection_string = "my-secure.connection/database";

let conn = OracleConnect::new(connection_string, username, password)?;
```

### SQLite 
You can open a SQLite connection in a db file or open a connection in memory.
```rust
use sql_tools::sql_variations::SQLiteConnect;

// Open connection from a file
let path = "<PATH_TO_DB_FILE>";
let conn = SQLiteConnect::from_path(path);

// Open connection from memory
let conn = SQLiteConnect::in_memory();
```

Once you established a connection type, you can use the various methods in this crate to interact with your database. These options are [select](#select), [update](#update), [insert](#insert), [create](#create), [delete](delete), and [alter](#alter). The data types that are supported by default can be found in the docs under the [ToSQLData](https://docs.rs/sql_tools/latest/sql_tools/data_types/trait.ToSQLData.html) trait. You can [implement ToSQLData for your own enum or struct](#ToSQLData) to make integration into your application easy.

## SQLDataTypes
This is the enum that is used to apply the proper type to the data that's being selected, updated, or inserted.
```rust
pub enum SQLDataTypes {
    Varchar(String),
    Number(i64),
    Float(f64),
    Date(chrono::NaiveDateTime), 
    NULL,
}
```

## SELECT
For the select method, you add the table you want to select from, then you can choose specific columns, functions, varchars, or everything from a particular table. To see these options more in-depth, look at the [`Column`](crate::statements::select::Column) enum. You can add a [`conjunction statement`](crate::query_conjunctions::QueryConjunctions) to filter out the rows you want, just like a SQL query.
```rust
let product_ids = WhereArg::Values(vec![1001.to_sql_fmt(), 4567.to_sql_fmt()]);
let state = WhereArg::Like("Tex%");
let cities = WhereArg::Values(vec![SQLDataTypes::Varchar("Austin"), SQLDataTypes::Varchar("Dallas")]);
let columns = vec![
    Column::Value(ColumnProps { name: "revenue as national_rev".to_string(), table: "national_sales".to_string() }),
    Column::Value(ColumnProps { name: "revenue as regional_rev".to_string(), table: "regional_sales".to_string() }),
    Column::Value(ColumnProps { name: "city".to_string(), table: "regional_sales".to_string() }),
    Column::Function("to_date('01/01/2000', 'mm/dd/YYYY')".to_string()),
];
let data: Vec<Vec<SQLDataTypes>> = conn
    .select("regional_sales", columns)
    .inner_join("national_sales", "product_id", "product_id")
    .where_in("product_id", product_ids)
    .and("national_sales.state", state) 
    .and_not("city", cities)
    .build()?;
data.iter().for_each(|row: &Vec<SQLDataTypes>| { println!("{:?}", row) });
```

## UPDATE
Updates a table's column(s) based on criteria set with an optional [`conjunction statement`](crate::query_conjunctions::QueryConjunctions). Updates can return Ok() or the number of rows that were updated.
```rust
let north_american_countries = vec!["Canada".to_sql_fmt(), "United States".to_sql_fmt(), "Mexico".to_sql_fmt()];
let na_countries_formatted = north_american_countries.iter().map(|value| value.to_sql_fmt()).collect::<Vec<SQLDataTypes>>();
let countries = WhereArg::Values(na_countries_formatted);
conn.update("global_sales")
    .set("continent", "North America")
    .where_in("country", countries)
    .build()?;
// If you want to get the number of rows that were updated
let count: usize = conn
    .update("global_sales")
    .set("continent", "North America")
    .where_in("country", countries)
    .build_return_count()?;
```

## INSERT
Inserts a grid (two-dimensional vector) of data into your database. Can take any type that has the [`ToSQLData`](#tosqldata) trait implemented. 
```rust
let data: Vec<Vec<&str>> = vec![
    vec!["Column_A", "Column_B", "Column_C"],
    vec!["a1", "b1", "c1"],
    vec!["a2", "b2", "c2"],
    vec!["a3", "b3", "c3"],
];

conn.insert("my_table", data)?.build()?;
```

If the table does not exist, you can add the `create_table()` method to automatically create the table.
```rust
conn.insert("my_table", data)?.create_table().build()?;
```

If you have a grid of strings that have integers, dates, etc.. that you want to be formatted properly before being inserted into a table then you want to add the `.format_grid_strings()` method.
```rust
let data: Vec<Vec<&str>> = vec![
    vec!["product_id", "date_sold", "price"],
    vec!["P-001", "09/15/2024", "54.99"],
    vec!["P-002", "10/22/2024", "19.99"],
    vec!["P-003", "11/04/2024", "39.99"],
];

conn.insert("sales_data", data)?
    // Will convert the "date_sold" column into chrono::NaiveDateTime
    // and the "price" column into f64.
    .format_grid_strings()?
    .build()?;
```

## CREATE
Creates a table using a vector of the `CreateColumns` struct and the `CreateDataTypes` to apply the correct types to the new columns.
```rust
let columns = vec![
        CreateColumns{ name: "Column_A".to_string(), data_type: CreateDataTypes::VARCHAR(20 as usize) },
        CreateColumns{ name: "Column_B".to_string(), data_type: CreateDataTypes::NUMBER },
        CreateColumns{ name: "Column_C".to_string(), data_type: CreateDataTypes::FLOAT },
    ];

    conn.create()
        .table("my_table", columns)
        .build()?;
```

You can add a column after you initiated the create table process.
```rust
let my_table: CreateTable = conn.create()
    .table("my_table", columns);

if add_date_col == true {
    my_table.add_column("Column_D".to_string(), CreateDataTypes::DATE);
}

my_table.build()?;
```

## DELETE
Deletes rows in a table based on the where methods added to the `DeleteProps`. If no where methods are added, it will delete all data in the table.
```rust
let terminated = WhereArg::Query("SELECT status FROM employment_statues WHERE status_type LIKE 'termina%'");
conn.delete("employee_data")
    .where_in("status", terminated)
    .build()?;
```

## ALTER
Alters a table by renaming it or adding, modifying, dropping, or renaming a column.
```rust
// renaming a table
conn.alter()
    .table("local_sales")
    .rename("regional_sales")
    .build()?;

// Adding a column
let column = AlterColumns {
    name: String::from("title"),
    data_type: CreateDataTypes::VARCHAR(10),
    default: Some(String::from("PMO")),
    not_null: true,
};
conn.alter()
    .table("employees")
    .add(vec![column])
    .build()?;

// Modifying a column (very similar to adding)
conn.alter()
    .table("employees")
    .modify(vec![column])
    .build()?; 

// Dropping a column
conn.alter()
    .table("sales")
    .drop("description")
    .build()?;

// Renaming a column
conn.alter()
    .table("sales")
    .rename_column("salesman", "employee")
    .build()?;
```

## Where
Conjunction statements are split into 4 categories via the `WhereArg` enum to prevent SQL injections, potential issues with NULL values, and for more intentional query structure.

- `Values` is a vector of any data type that has is implemented for the `ToSQLData` trait. This would be used as if you have a basic WHERE clause that you have set values for.
```rust
.where_in("column", WhereArg::Values(vec!["one".to_sql_fmt(), "two".to_sql_fmt(), "three".to_sql_fmt()]))
```
```sql
WHERE column IN ('one', 'two', 'three');
```
- `Like` is used the same way as a SQL LIKE statement.
```rust
.where_in("column", WhereArg::Like("Hello Wor%"))
```
```sql
WHERE column IN ('Hello Wor%');
```
- `Query` is when you want to select values from another table.
```rust
.where_in("column", WhereArg::Query("SELECT value FROM another_table"))
```
```sql
WHERE column IN (SELECT value FROM another_table)
```
- `NULL` is for selecting NULL values.
```rust
.where_in("column", WhereArg::NULL)
```
```sql
WHERE column IS NULL
```

## ToSQLData
`ToSQLData` is the trait that is used to convert various data types to `SQLDataTypes`. 

To implement a local enum: 
```rust
enum MyEnum {
    Name(String),
    Age(i64)
}

impl ToSQLData for MyEnum {
    fn to_sql_fmt(&self) -> SQLDataTypes {
        match self {
            MyEnum::Name(val) => SQLDataTypes::Varchar(val.into()),
            MyEnum::Age(val) => SQLDataTypes::Number(val.into()),
        }
    }
}
```

To implement a foreign enum:
```rust
use some_crate::SomeForeignType;

struct MyType<'a>(&'a SomeForeignType);

impl ToSQLData for MyType<'_> {
    fn to_sql_fmt(&'_ self) -> SQLDataTypes {
        match self {
            SomeForeignType::Int(val) => SQLDataTypes::Number(*val),
            SomeForeignType::Float(val) => SQLDataTypes::Float(*val),
            SomeForeignType::String(val) => SQLDataTypes::Varchar(val.to_owned()),
            SomeForeignType::None => SQLDataTypes::NULL,
        }
    }
}
```
