# QueryBuilder
✅- Make every method a borrow self so one connection struct can be used multiple times
- Add SQLite support
- Add delete method

# Insert
- A 2nd insert method that only takes data and excludes header
- Add method to add/replace header
⏸️- `.format_data()` to auto convert inserted data into appropriate data type
- Single thread build method
- Add a progress bar option
- Add method that will catch if table does not exist then abort method

# Select 
- Single thread build method
- Add a progress bar option

# CREATE
- Create view method
- CreateDataTypes needs to be on par with SQLDataType

# Where Clause
✅- change `.filter()` to `.where_in()` and `.where_not()`
✅- change `.and()` `.or()` to  `.and()`/`.and_not()` and `.or()`/`.or_not()`
✅- let the values types be the FormatData generic
- Add a date range filter

# SQLDataType
- Add XML type
- Time type (chrono::NaiveTime)
- RowID
- convert NUMBER to u64 or usize