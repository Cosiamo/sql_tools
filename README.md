# SQL Tools
A rust crate meant to make creating SQL queries easy. The goal is to have multiple different versions of SQL (SQLite, Postgres, etc..) however it's only Oracle SQL right now.

# How To Use
To start using SQL Tools, you need a new connection.

```rust
use sql_tools::sql_variations::OracleConnect;

let username = "new_user";
let password = "password123!";
let connection_string = "my-secure.connection/database";

let conn = OracleConnect::new(connection_string, username, password);
```

Once you established a connection type, you can use the various methods in this crate to interact with your database. These options are [select](#select), [update](#update), [insert](#insert), and [create](#create).

## SELECT
For the select method, you add the table you want to select from, then the columns in a vector. If you want to select all, simply input `vec!["*"]`. You can add a [where](#where), [and](#and), and/or [or](#or) to filter out the rows you want, just like writing a SQL query.
```rust
let conn = OracleConnect::new(connection_string, username, password);
let data: Vec<Vec<SQLDataTypes>> = conn
    .select("regional_sales", vec!["product_id", "revenue", "sale_date"])
    .where_in("product_id", vec!["1001", "4567"])
    .build();
data.iter().for_each(|row: &Vec<SQLDataTypes>| { println!("{:?}", row) });
```