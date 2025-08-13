# Changelog

All notable changes to this project will be documented in this file.
Dates are labeled YYYY-MM-DD.

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