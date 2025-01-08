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
✅- Single thread build method
- Add a progress bar option
✅- Order by
- .distinct() method

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
- .to_num(), .to_float(), .to_date(), etc...

# Other
✅- Test if connection works or not
- Redo how the parallel select works
    - COUNT operations do not properly count
    - Does not return correct order (not as important)
✅- Add ALTER
- Add dual method so you can manipulate vector grids with sql