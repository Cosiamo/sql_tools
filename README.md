# SQL Tools
A rust crate meant to make creating SQL queries easy. The goal is to have multiple different versions of SQL (SQLite, Postgres, etc..) however it's only Oracle SQL right now. The original crate I made that this is an evolution of is [oracle_sql_tools](https://crates.io/crates/oracle_sql_tools).

# How To Use
In your `cargo.toml` file:
```toml
[dependencies]
sql_tools = "0.1.0"
# chrono is required if you're working with dates 
chrono = "0.4.26" 
```

To start using SQL Tools, you need a new connection.
```rust
use sql_tools::sql_variations::OracleConnect;

let username = "new_user";
let password = "password123!";
let connection_string = "my-secure.connection/database";

let conn = OracleConnect::new(connection_string, username, password);
```

Once you established a connection type, you can use the various methods in this crate to interact with your database. These options are [select](#select), [update](#update), [insert](#insert), and [create](#create).

## SQLDataTypes
This is the enum that is used to apply the proper data type to the data that's being selected, updated, or inserted.
```rust
pub enum SQLDataTypes {
    VARCHAR(String),
    NUMBER(i64),
    FLOAT(f64),
    DATE(chrono::NaiveDateTime), 
    NULL,
}
```

## SELECT
For the select method, you add the table you want to select from, then the columns in a vector. If you want to select all, simply input `vec!["*"]`. You can add a [where clause](#where) to filter out the rows you want, just like writing a SQL query.
```rust
let conn = OracleConnect::new(connection_string, username, password);
let data: Vec<Vec<SQLDataTypes>> = conn
    .select("regional_sales", vec!["product_id", "revenue", "sale_date"])
    .where_in("product_id", vec!["1001", "4567"])
    .and_not("city", vec!["Austin", "Dallas"])
    .build()?;
data.iter().for_each(|row: &Vec<SQLDataTypes>| { println!("{:?}", row) });
```

## UPDATE
Updates a table's column(s) based on criteria set in the optional [where clauses](#where). Updates can return Ok() or the number of rows that were updated.
```rust
let conn = OracleConnect::new(connection_string, username, password);
conn.update("global_sales")
    .set("continent", "North America")
    .where_in("country", vec!["Canada", "United States", "Mexico"])
    .build()?;
// If you want to get the number of rows that were updated
let count: usize = conn
    .update("global_sales")
    .set("continent", "North America")
    .where_in("country", vec!["Canada", "United States", "Mexico"])
    .build_return_count()?;
```

## INSERT
Inserts a grid (two-dimensional vector) of data into your database. Can take any type that has the [`ToSQLData`](#tosqldata) trait implemented. If the table does not exist, it will automatically create a new table (will have an abort option in a future update).
```rust
let conn = OracleConnect::new(connection_string, username, password);
let data: Vec<Vec<&str>> = vec![
    vec!["Column_A", "Column_B", "Column_C"],
    vec!["a1", "b1", "c1"],
    vec!["a2", "b2", "c2"],
    vec!["a3", "b3", "c3"],
];

conn.insert("my_table", data).build()?;
```

If you have a grid of strings that have integers, dates, etc.. that you want to be formatted properly before being inserted into a table then you want to add the `.format_grid_strings()` method.
```rust
let data: Vec<Vec<&str>> = vec![
    vec!["product_id", "date_sold", "price"],
    vec!["P-001", "09/15/2024", "54.99"],
    vec!["P-002", "10/22/2024", "19.99"],
    vec!["P-003", "11/04/2024", "39.99"],
];

conn.insert("sales_data", data)
    // Will convert the "date_sold" column into chrono::NaiveDateTime
    // and the "price" column into f64.
    .format_grid_strings()?
    .build()?;
```

## CREATE
Creates a table using a vector of the `CreateColumns` struct and the `CreateDataTypes` to apply the correct types to the new columns.
```rust
let conn = OracleConnect::new(connection_string, username, password);
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

## ToSQLData
`ToSQLData` is the trait that is implemented for the `insert()` method which is used to convert various data types to `SQLDataTypes`. 

To implement a local enum: 
```rust
enum MyEnum {
    Name(String),
    Age(i64)
}

impl ToSQLData for MyEnum {
    fn fmt_data(self) -> SQLDataTypes {
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
    fn fmt_data(self) -> SQLDataTypes {
        match self {
            MyType(SomeForeignType::Int(val)) => SQLDataTypes::Number(*val),
            MyType(SomeForeignType::Float(val)) => SQLDataTypes::Float(*val),
            MyType(SomeForeignType::String(val)) => SQLDataTypes::Varchar(val.to_owned()),
            MyType(SomeForeignType::None) => SQLDataTypes::NULL,
        }
    }
}
```