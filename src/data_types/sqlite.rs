use rusqlite::types::{ToSqlOutput, Value};

use super::SQLDataTypes;

impl rusqlite::types::FromSql for SQLDataTypes {
    fn column_result(val: rusqlite::types::ValueRef<'_>) -> rusqlite::types::FromSqlResult<Self> {
        Ok(match val {
            rusqlite::types::ValueRef::Null => SQLDataTypes::NULL,
            rusqlite::types::ValueRef::Integer(val) => SQLDataTypes::Number(val),
            rusqlite::types::ValueRef::Real(val) => SQLDataTypes::Float(val),
            rusqlite::types::ValueRef::Text(val) => {
                let v = String::from_utf8_lossy(val);
                SQLDataTypes::Varchar(v.to_string())
            }
            rusqlite::types::ValueRef::Blob(val) => {
                let v = String::from_utf8_lossy(val);
                SQLDataTypes::Varchar(v.to_string())
            }
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
