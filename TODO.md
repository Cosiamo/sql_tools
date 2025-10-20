# Insert
- A 2nd insert method that only takes data and excludes header
- Add method to add/replace header

# Select 

# CREATE
- Create view method
- CreateDataTypes needs to be on par with SQLDataType
    - See if I can make them the same struct without having conflicts

# Where Clause
- Add a date range filter

# SQLDataType
- Add XML type
- Time type (chrono::NaiveTime)
- convert NUMBER to u64 or usize
- .to_num(), .to_float(), .to_date(), etc...

# Other
- Redo how the parallel select works
    - COUNT operations do not properly count
    - Does not return correct order (not as important)
- Add dual method so you can manipulate vector grids with sql
- Change function and method args to borrows (might not need to do this with 'self')