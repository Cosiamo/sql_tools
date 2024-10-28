# QueryBuilder
✅- Make every method a borrow self so one connection struct can be used multiple times
- Add SQLite support

# Insert
- Create a 2nd insert method that only takes data and excludes header
- Add method to add/replace header
- `.format_data()` to auto convert inserted data into appropriate data type
- Create a single thread build method

# Select 
- Create a single thread build method

# Where Clause
✅- change `.filter()` to `.where_in()` and `.where_not()`
✅- change `.and()` `.or()` to  `.and()`/`.and_not()` and `.or()`/`.or_not()`
✅- let the values types be the FormatData generic
- Add a date range filter