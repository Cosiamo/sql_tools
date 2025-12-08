# Changelog

All notable changes to this project will be documented in this file.

How I determine version numbers for this project: 
- Major version: substantial changes such as implementing a new SQL type or a rewrite of the library.
- Minor version: breaking changes such as changing the name of a method or changing method arguments. Minor versions may also include major, non-breaking changes such as adding new features or improving existing ones.
- Patch version: non-breaking changes such as fixing a bug, minor performance improvements, or improving documentation.

Dates are labeled YYYY-MM-DD.

## [0.11.3] - 2025-12-08
### Fixed 
- return_header method for select doesn't return the column alias attached anymore. Just the column name.
- Fixed issue where insert method for SQLite would fail when passing using columns from another database.

## [0.11.2] - 2025-12-03
### Reverted
- Reverted the decision to borrow the Column struct in the select method.

## [0.11.1] - 2025-12-03
### Changed
- Can now pass an empty string into the ColumnProps table field for conjunctions. This is a temporary fix for passing functions, such as `UPPER(column_name)`, as the column in conjunctions. Will work on a better solution in the future.

## [0.11.0] - 2025-12-02
### Changed
- Conjunction methods use ColumnProps as the argument for columns instead or strings.
- Select method uses borrowed ColumnProps instead of consuming them.

### Added
- Derived Clone in the Column and ColumnProps struct

## [0.10.0] - 2025-12-01
### Changed
- The columns vector for the select method uses SQLDataType instead of String

## [0.9.0] - 2025-11-17
### Added
- In memory SQLite database

### Changed
- `SQLiteConnect::new_path()` is now `SQLiteConnect::from_path()`
- `fmt_data()` is now `to_sql_fmt()`

### Improved
- Inlined new connection methods

## [0.8.1] - 2025-11-12
### Added
- Impl ToSqlData trait for `Option<SQLDataType>` and `Box<Option<SQLDataType>>` 

## [0.8.0] - 2025-11-04
### Added
- WhereArgs enum

### Removed
- where_like, where_null, where_not_like, where_not_null, and_like, and_null, and_not_like, and_not_null, or_like, or_null, or_not_like, or_not_null

### Changed
- where_clause module to query_conjunctions
- WhereClauseBuilder to QueryConjunctions
- where_in, where_not, and, and_not, or, or_not all take a WhereArgs argument instead of Vec<T> where T: ToSQLData
- where_in and where_not no longer have their own individual traits for each statement type, instead they live in the QueryConjunctions trait

## [0.7.2] - 2025-10-27
### Improved
- Improved speed and memory usage for multithreaded select 

## [0.7.1] - 2025-10-23
### Fixed
- table_info method for SQLite was broken as a result of the table.column update in 0.7.0, fixed this to allow the function to work as intended

## [0.7.0] - 2025-10-20
### Added
- Added the ability to return header names via return_header() in select()
- More documentation for delete, select, create, and where clause methods

### Changed
- SQLVariations is now SQLImplementations to follow common naming conventions
- .create().add_column() now borrows self
- replaced fmt_data() with fmt_data_borrowed()'s functionality to avoid self consumption in method and rm fmt_data_borrowed()
- format_grid_strings() uses self instead of creating new struct
- made oracle col headers public
- Removed the `Table` struct and reverted all method params with this data type back to `&str`

## [0.6.0] - 2025-08-13
### Added
- Where "like" and "not like" statements
- Changed the input to the `update` method to a `Table` struct

## [0.5.1] - 2025-08-04
- Restructured where the sql variations are executing their statements (for easier maintenance and upgradeability)

## [0.5.0] - 2025-07-23
### Added
- Renamed the 'Clauses' module to the more appropriately named 'Statements'
- Added joins to the select statement

### Fixed
- Fixed single threaded builds not recognizing "*"
