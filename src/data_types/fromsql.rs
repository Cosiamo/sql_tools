use chrono::NaiveDateTime;
use oracle::sql_type::{FromSql, OracleType, ToSql};
use rusqlite::types::{ToSqlOutput, Value};

use super::SQLDataTypes;

impl FromSql for SQLDataTypes {
    fn from_sql(val: &oracle::SqlValue) -> oracle::Result<Self>
    where
        Self: Sized {
            if val.is_null()? { return Ok(SQLDataTypes::NULL) }

            Ok(match val.oracle_type()? {
                OracleType::Varchar2(_) => SQLDataTypes::Varchar(val.get::<String>()?),
                OracleType::NVarchar2(_) => SQLDataTypes::Varchar(val.get::<String>()?),
                OracleType::Char(_) => SQLDataTypes::Varchar(val.get::<String>()?),
                OracleType::NChar(_) => SQLDataTypes::Varchar(val.get::<String>()?),
                OracleType::Rowid => SQLDataTypes::Varchar(val.get::<String>()?),
                OracleType::Raw(_) => SQLDataTypes::Varchar(val.get::<String>()?),
                OracleType::BinaryFloat => SQLDataTypes::Float(val.get::<f64>()?),
                OracleType::BinaryDouble => SQLDataTypes::Number(val.get::<i64>()?),
                OracleType::Number(_, _) => SQLDataTypes::Number(val.get::<i64>()?),
                OracleType::Float(_) => SQLDataTypes::Float(val.get::<f64>()?),
                OracleType::Date => SQLDataTypes::Date(val.get::<NaiveDateTime>()?),
                OracleType::Timestamp(_) => SQLDataTypes::Varchar(val.get::<String>()?),
                OracleType::TimestampTZ(_) => SQLDataTypes::Varchar(val.get::<String>()?),
                OracleType::TimestampLTZ(_) => SQLDataTypes::Varchar(val.get::<String>()?),
                OracleType::IntervalDS(_, _) => SQLDataTypes::Varchar(val.get::<String>()?),
                OracleType::IntervalYM(_) => SQLDataTypes::Varchar(val.get::<String>()?),
                OracleType::CLOB => SQLDataTypes::Varchar(val.get::<String>()?),
                OracleType::NCLOB => SQLDataTypes::Varchar(val.get::<String>()?),
                OracleType::BLOB => SQLDataTypes::Varchar(val.get::<String>()?),
                OracleType::BFILE => SQLDataTypes::Varchar(val.get::<String>()?),
                OracleType::RefCursor => SQLDataTypes::Varchar(val.get::<String>()?),
                OracleType::Boolean => SQLDataTypes::Varchar(val.get::<String>()?),
                OracleType::Object(_) => SQLDataTypes::Varchar(val.get::<String>()?),
                OracleType::Long => SQLDataTypes::Varchar(val.get::<String>()?),
                OracleType::LongRaw => SQLDataTypes::Varchar(val.get::<String>()?),
                OracleType::Json => SQLDataTypes::Varchar(val.get::<String>()?),
                OracleType::Xml => SQLDataTypes::Varchar(val.get::<String>()?),
                OracleType::Int64 =>  SQLDataTypes::Number(val.get::<i64>()?),
                OracleType::UInt64 =>  SQLDataTypes::Number(val.get::<i64>()?),
            })
    }
}

impl ToSql for SQLDataTypes {
    fn oratype(&self, conn: &oracle::Connection) -> oracle::Result<OracleType> {
        match self {
            SQLDataTypes::Varchar(val) => val.oratype(conn),
            SQLDataTypes::Number(val) => val.oratype(conn),
            SQLDataTypes::Float(val) => val.oratype(conn),
            SQLDataTypes::Date(val) => val.oratype(conn),
            SQLDataTypes::NULL => "".oratype(conn),
        }
    }

    fn to_sql(&self, val: &mut oracle::SqlValue) -> oracle::Result<()> {
        match self {
            SQLDataTypes::Varchar(v) => val.set(v),
            SQLDataTypes::Number(v) => val.set(v),
            SQLDataTypes::Float(v) => val.set(v),
            SQLDataTypes::Date(v) => val.set(v),
            SQLDataTypes::NULL => val.set_null(),
        }
    }
}

impl rusqlite::types::FromSql for SQLDataTypes {
    fn column_result(val: rusqlite::types::ValueRef<'_>) -> rusqlite::types::FromSqlResult<Self> {
        Ok(match val {
            rusqlite::types::ValueRef::Null => SQLDataTypes::NULL,
            rusqlite::types::ValueRef::Integer(val) => SQLDataTypes::Number(val),
            rusqlite::types::ValueRef::Real(val) => SQLDataTypes::Float(val),
            rusqlite::types::ValueRef::Text(val) => {
                let v = String::from_utf8_lossy(val);
                SQLDataTypes::Varchar(v.to_string())
            },
            rusqlite::types::ValueRef::Blob(val) => {
                let v = String::from_utf8_lossy(val);
                SQLDataTypes::Varchar(v.to_string())
            },
        })
    }
}

impl rusqlite::types::ToSql for SQLDataTypes {
    fn to_sql(&self) -> rusqlite::Result<ToSqlOutput<'_>> {
        Ok(match self {
            SQLDataTypes::Varchar(val) => ToSqlOutput::Owned(Value::Text(val.to_string())),
            SQLDataTypes::Number(val) => ToSqlOutput::Owned(Value::Integer(*val)),
            SQLDataTypes::Float(val) => ToSqlOutput::Owned(Value::Real(*val)),
            SQLDataTypes::Date(val) => ToSqlOutput::Owned(Value::Text(val.to_string())),
            SQLDataTypes::NULL => ToSqlOutput::Owned(Value::Null),
        })
    }
}