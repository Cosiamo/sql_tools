use core::fmt;

use chrono::NaiveDateTime;

pub mod implement;
pub mod fromsql;

#[derive(Debug, Clone, PartialEq)]
pub enum SQLDataTypes {
    VARCHAR(String),
    INT(i64),
    FLOAT(f64),
    DATE(NaiveDateTime),
    NULL,
}

impl fmt::Display for SQLDataTypes {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SQLDataTypes::VARCHAR(val) => write!(f, "{}", val),
            SQLDataTypes::INT(val) => write!(f, "{}", val),
            SQLDataTypes::FLOAT(val) => write!(f, "{}", val),
            SQLDataTypes::DATE(val) => write!(f, "{}", val),
            SQLDataTypes::NULL => write!(f, ""),
        }
    }
}

/// A trait that formats the input data to match [`SQLDataTypes`]
///
/// Already implemented for `&[u8]`, `Vec<u8>`, `&str`, 'String', 'i8', 'i16', 'i32', 'i64', 'f32', 'f64', and [`chrono::NaiveDateTime`], as well as, their Option<> variants
///
/// To implement a local enum: 
///
/// ```no_run
/// enum MyEnum {
///     STRING(String),
///     NUMBER(i64)
/// }
///
/// impl ToSQLData for MyEnum {
///     fn fmt_data(self) -> SQLDataTypes {
///         match self {
///             MyEnum::STRING(val) => SQLDataTypes::VARCHAR(val.into()),
///             MyEnum::NUMBER(val) => SQLDataTypes::INT(val.into()),
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
///     fn fmt_data(self) -> SQLDataTypes {
///         match self {
///             MyType(SomeForeignType::Int(val)) => SQLDataTypes::INT(*val),
///             MyType(SomeForeignType::Float(val)) => SQLDataTypes::FLOAT(*val),
///             MyType(SomeForeignType::String(val)) => SQLDataTypes::VARCHAR(val.to_owned()),
///             MyType(SomeForeignType::None) => SQLDataTypes::NULL,
///         }
///     }
/// }
/// ```
pub trait ToSQLData { 
    fn fmt_data(self) -> SQLDataTypes; 
    fn fmt_data_borrowed(&self) -> SQLDataTypes;
}