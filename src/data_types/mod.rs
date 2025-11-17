use core::fmt;

use chrono::NaiveDateTime;

pub mod implement;
pub mod oracle;
pub mod sqlite;

#[derive(Debug, Clone, PartialEq)]
pub enum SQLDataTypes {
    Varchar(String),
    Number(i64),
    Float(f64),
    Date(NaiveDateTime),
    NULL,
}

impl fmt::Display for SQLDataTypes {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SQLDataTypes::Varchar(val) => write!(f, "{}", val),
            SQLDataTypes::Number(val) => write!(f, "{}", val),
            SQLDataTypes::Float(val) => write!(f, "{}", val),
            SQLDataTypes::Date(val) => write!(f, "{}", val),
            SQLDataTypes::NULL => write!(f, ""),
        }
    }
}

/// A trait that formats the input data to match [`SQLDataTypes`]
///
/// Already implemented for `&[u8]`, `Vec<u8>`, `&str`,`String`, `i8`, `i16`, `i32`, `i64`, `f32`, `f64`, and [`chrono::NaiveDateTime`], as well as, their Option<> and Box<> variants
///
/// To implement a local enum:
///
/// ```no_run
/// enum MyEnum {
///     Name(String),
///     Age(i64)
/// }
///
/// impl ToSQLData for MyEnum {
///     fn to_sql_fmt(&self) -> SQLDataTypes {
///         match self {
///             MyEnum::Name(val) => SQLDataTypes::Varchar(val.into()),
///             MyEnum::Age(val) => SQLDataTypes::Number(val.into()),
///         }
///     }
/// }
/// ```
///
/// To implement a foreign enum:
///
/// ```no_run
/// use some_crate::SomeForeignType;
///
/// struct MyType<'a>(&'a SomeForeignType);
///
/// impl ToSQLData for MyType<'_> {
///     fn to_sql_fmt(&'_ self) -> SQLDataTypes {
///         match self {
///             MyType(SomeForeignType::Int(val)) => SQLDataTypes::Number(*val),
///             MyType(SomeForeignType::Float(val)) => SQLDataTypes::Float(*val),
///             MyType(SomeForeignType::String(val)) => SQLDataTypes::Varchar(val.to_owned()),
///             MyType(SomeForeignType::None) => SQLDataTypes::NULL,
///         }
///     }
/// }
/// ```
///
pub trait ToSQLData {
    fn to_sql_fmt(&self) -> SQLDataTypes;
}
