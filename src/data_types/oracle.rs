use chrono::NaiveDateTime;
use oracle::sql_type::{FromSql, OracleType, ToSql};

use super::SQLDataTypes;

impl FromSql for SQLDataTypes {
    fn from_sql(val: &oracle::SqlValue) -> oracle::Result<Self>
    where
        Self: Sized,
    {
        if val.is_null()? {
            return Ok(SQLDataTypes::NULL);
        }

        Ok(match val.oracle_type()? {
            OracleType::Varchar2(_) => SQLDataTypes::Varchar(val.get::<String>()?),
            OracleType::NVarchar2(_) => SQLDataTypes::Varchar(val.get::<String>()?),
            OracleType::Char(_) => SQLDataTypes::Varchar(val.get::<String>()?),
            OracleType::NChar(_) => SQLDataTypes::Varchar(val.get::<String>()?),
            OracleType::Rowid => SQLDataTypes::Varchar(val.get::<String>()?),
            OracleType::Raw(_) => SQLDataTypes::Varchar(val.get::<String>()?),
            OracleType::BinaryFloat => SQLDataTypes::Float(val.get::<f64>()?),
            OracleType::BinaryDouble => SQLDataTypes::Float(val.get::<f64>()?),
            OracleType::Number(_, _) => {
                let buff = val.get::<String>()?;
                if buff.contains(".") {
                    SQLDataTypes::Float(val.get::<f64>()?)
                } else {
                    SQLDataTypes::Number(val.get::<i64>()?)
                }
            }
            OracleType::Float(_) => SQLDataTypes::Float(val.get::<f64>()?),
            OracleType::Date => SQLDataTypes::Date(val.get::<NaiveDateTime>()?),
            OracleType::Timestamp(_) => SQLDataTypes::Date(val.get::<NaiveDateTime>()?),
            OracleType::TimestampTZ(_) => SQLDataTypes::Date(val.get::<NaiveDateTime>()?),
            OracleType::TimestampLTZ(_) => SQLDataTypes::Date(val.get::<NaiveDateTime>()?),
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
            OracleType::Int64 => {
                let buff = val.get::<String>()?;
                if buff.contains(".") {
                    SQLDataTypes::Float(val.get::<f64>()?)
                } else {
                    SQLDataTypes::Number(val.get::<i64>()?)
                }
            }
            OracleType::UInt64 => {
                let buff = val.get::<String>()?;
                if buff.contains(".") {
                    SQLDataTypes::Float(val.get::<f64>()?)
                } else {
                    SQLDataTypes::Number(val.get::<i64>()?)
                }
            }
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
            // Null match is practically worthless (real integration at batch_bind in insert).
            // Spent hours trying to find a clever way integrate NULL type with OracleType
            // but they structured it in a way that they need an Option<T> but it couldn't return None::<ToSql>
            // because there's a ToSqlNull trait which is a pain in the ass to implement for an enum.
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
