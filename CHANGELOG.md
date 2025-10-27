# Changelog

All notable changes to this project will be documented in this file.
Dates are labeled YYYY-MM-DD.

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