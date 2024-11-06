use chrono::NaiveDateTime;
use oracle::sql_type::FromSql;

use super::SQLDataTypes;


impl FromSql for SQLDataTypes {
    fn from_sql(val: &oracle::SqlValue) -> oracle::Result<Self>
    where
        Self: Sized {
            if val.is_null()? { return Ok(SQLDataTypes::NULL) }

            let t = match val.oracle_type()? {
                oracle::sql_type::OracleType::Varchar2(_) => SQLDataTypes::Varchar(val.get::<String>()?),
                oracle::sql_type::OracleType::NVarchar2(_) => SQLDataTypes::Varchar(val.get::<String>()?),
                oracle::sql_type::OracleType::Char(_) => SQLDataTypes::Varchar(val.get::<String>()?),
                oracle::sql_type::OracleType::NChar(_) => SQLDataTypes::Varchar(val.get::<String>()?),
                oracle::sql_type::OracleType::Rowid => SQLDataTypes::Varchar(val.get::<String>()?),
                oracle::sql_type::OracleType::Raw(_) => SQLDataTypes::Varchar(val.get::<String>()?),
                oracle::sql_type::OracleType::BinaryFloat => SQLDataTypes::Float(val.get::<f64>()?),
                oracle::sql_type::OracleType::BinaryDouble => SQLDataTypes::Number(val.get::<i64>()?),
                oracle::sql_type::OracleType::Number(_, _) => SQLDataTypes::Number(val.get::<i64>()?),
                oracle::sql_type::OracleType::Float(_) => SQLDataTypes::Float(val.get::<f64>()?),
                oracle::sql_type::OracleType::Date => SQLDataTypes::Date(val.get::<NaiveDateTime>()?),
                oracle::sql_type::OracleType::Timestamp(_) => SQLDataTypes::Date(val.get::<NaiveDateTime>()?),
                oracle::sql_type::OracleType::TimestampTZ(_) => SQLDataTypes::Date(val.get::<NaiveDateTime>()?),
                oracle::sql_type::OracleType::TimestampLTZ(_) => SQLDataTypes::Date(val.get::<NaiveDateTime>()?),
                oracle::sql_type::OracleType::IntervalDS(_, _) => SQLDataTypes::Date(val.get::<NaiveDateTime>()?),
                oracle::sql_type::OracleType::IntervalYM(_) => SQLDataTypes::Date(val.get::<NaiveDateTime>()?),
                oracle::sql_type::OracleType::CLOB => SQLDataTypes::Varchar(val.get::<String>()?),
                oracle::sql_type::OracleType::NCLOB => SQLDataTypes::Varchar(val.get::<String>()?),
                oracle::sql_type::OracleType::BLOB => SQLDataTypes::Varchar(val.get::<String>()?),
                oracle::sql_type::OracleType::BFILE => SQLDataTypes::Varchar(val.get::<String>()?),
                oracle::sql_type::OracleType::RefCursor => SQLDataTypes::Varchar(val.get::<String>()?),
                oracle::sql_type::OracleType::Boolean => SQLDataTypes::Varchar(val.get::<String>()?),
                oracle::sql_type::OracleType::Object(_) => SQLDataTypes::Varchar(val.get::<String>()?),
                oracle::sql_type::OracleType::Long => SQLDataTypes::Varchar(val.get::<String>()?),
                oracle::sql_type::OracleType::LongRaw => SQLDataTypes::Varchar(val.get::<String>()?),
                oracle::sql_type::OracleType::Json => SQLDataTypes::Varchar(val.get::<String>()?),
                oracle::sql_type::OracleType::Xml => SQLDataTypes::Varchar(val.get::<String>()?),
                oracle::sql_type::OracleType::Int64 =>  SQLDataTypes::Number(val.get::<i64>()?),
                oracle::sql_type::OracleType::UInt64 =>  SQLDataTypes::Number(val.get::<i64>()?),
            };
            Ok(t)
    }
}