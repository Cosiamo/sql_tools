use chrono::NaiveDateTime;
use oracle::sql_type::FromSql;

use super::SQLDataTypes;


impl FromSql for SQLDataTypes {
    fn from_sql(val: &oracle::SqlValue) -> oracle::Result<Self>
    where
        Self: Sized {
            if val.is_null()? { return Ok(SQLDataTypes::NULL) }

            let t = match val.oracle_type()? {
                oracle::sql_type::OracleType::Varchar2(_) => SQLDataTypes::VARCHAR(val.get::<String>()?),
                oracle::sql_type::OracleType::NVarchar2(_) => SQLDataTypes::VARCHAR(val.get::<String>()?),
                oracle::sql_type::OracleType::Char(_) => SQLDataTypes::VARCHAR(val.get::<String>()?),
                oracle::sql_type::OracleType::NChar(_) => SQLDataTypes::VARCHAR(val.get::<String>()?),
                oracle::sql_type::OracleType::Rowid => SQLDataTypes::VARCHAR(val.get::<String>()?),
                oracle::sql_type::OracleType::Raw(_) => SQLDataTypes::VARCHAR(val.get::<String>()?),
                oracle::sql_type::OracleType::BinaryFloat => SQLDataTypes::FLOAT(val.get::<f64>()?),
                oracle::sql_type::OracleType::BinaryDouble => SQLDataTypes::NUMBER(val.get::<i64>()?),
                oracle::sql_type::OracleType::Number(_, _) => SQLDataTypes::NUMBER(val.get::<i64>()?),
                oracle::sql_type::OracleType::Float(_) => SQLDataTypes::FLOAT(val.get::<f64>()?),
                oracle::sql_type::OracleType::Date => SQLDataTypes::DATE(val.get::<NaiveDateTime>()?),
                oracle::sql_type::OracleType::Timestamp(_) => SQLDataTypes::DATE(val.get::<NaiveDateTime>()?),
                oracle::sql_type::OracleType::TimestampTZ(_) => SQLDataTypes::DATE(val.get::<NaiveDateTime>()?),
                oracle::sql_type::OracleType::TimestampLTZ(_) => SQLDataTypes::DATE(val.get::<NaiveDateTime>()?),
                oracle::sql_type::OracleType::IntervalDS(_, _) => SQLDataTypes::DATE(val.get::<NaiveDateTime>()?),
                oracle::sql_type::OracleType::IntervalYM(_) => SQLDataTypes::DATE(val.get::<NaiveDateTime>()?),
                oracle::sql_type::OracleType::CLOB => SQLDataTypes::VARCHAR(val.get::<String>()?),
                oracle::sql_type::OracleType::NCLOB => SQLDataTypes::VARCHAR(val.get::<String>()?),
                oracle::sql_type::OracleType::BLOB => SQLDataTypes::VARCHAR(val.get::<String>()?),
                oracle::sql_type::OracleType::BFILE => SQLDataTypes::VARCHAR(val.get::<String>()?),
                oracle::sql_type::OracleType::RefCursor => SQLDataTypes::VARCHAR(val.get::<String>()?),
                oracle::sql_type::OracleType::Boolean => SQLDataTypes::VARCHAR(val.get::<String>()?),
                oracle::sql_type::OracleType::Object(_) => SQLDataTypes::VARCHAR(val.get::<String>()?),
                oracle::sql_type::OracleType::Long => SQLDataTypes::VARCHAR(val.get::<String>()?),
                oracle::sql_type::OracleType::LongRaw => SQLDataTypes::VARCHAR(val.get::<String>()?),
                oracle::sql_type::OracleType::Json => SQLDataTypes::VARCHAR(val.get::<String>()?),
                oracle::sql_type::OracleType::Xml => SQLDataTypes::VARCHAR(val.get::<String>()?),
                oracle::sql_type::OracleType::Int64 =>  SQLDataTypes::NUMBER(val.get::<i64>()?),
                oracle::sql_type::OracleType::UInt64 =>  SQLDataTypes::NUMBER(val.get::<i64>()?),
            };
            Ok(t)
    }
}