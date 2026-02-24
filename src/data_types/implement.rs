use chrono::NaiveDateTime;

use super::{SQLDataTypes, ToSQLData};

impl ToSQLData for SQLDataTypes {
    fn to_sql_fmt(&self) -> Self {
        self.to_owned()
    }
}
impl ToSQLData for Option<SQLDataTypes> {
    fn to_sql_fmt(&self) -> SQLDataTypes {
        match self {
            Some(val) => val.to_owned(),
            None => SQLDataTypes::NULL,
        }
    }
}
impl ToSQLData for Box<SQLDataTypes> {
    fn to_sql_fmt(&self) -> SQLDataTypes {
        *self.to_owned()
    }
}
impl ToSQLData for Option<Box<SQLDataTypes>> {
    fn to_sql_fmt(&self) -> SQLDataTypes {
        match self {
            Some(val) => *val.to_owned(),
            None => SQLDataTypes::NULL,
        }
    }
}

fn bytes_to_varchar(bytes: &[u8]) -> SQLDataTypes {
    let utf8_string = String::from_utf8_lossy(bytes).replace(|c: char| !c.is_ascii(), "");
    SQLDataTypes::Varchar(utf8_string)
}

fn vec_u8_to_varchar(bytes: Vec<u8>) -> SQLDataTypes {
    let utf8_string = String::from_utf8(bytes)
        .map_err(|non_utf8| String::from_utf8_lossy(non_utf8.as_bytes()).into_owned())
        .unwrap();
    SQLDataTypes::Varchar(utf8_string)
}

impl ToSQLData for &[u8] {
    fn to_sql_fmt(&self) -> SQLDataTypes {
        bytes_to_varchar(self)
    }
}
impl ToSQLData for Vec<u8> {
    fn to_sql_fmt(&self) -> SQLDataTypes {
        vec_u8_to_varchar(self.to_vec())
    }
}
impl ToSQLData for Option<&[u8]> {
    fn to_sql_fmt(&self) -> SQLDataTypes {
        match self {
            Some(val) => bytes_to_varchar(val),
            None => SQLDataTypes::NULL,
        }
    }
}
impl ToSQLData for Option<Vec<u8>> {
    fn to_sql_fmt(&self) -> SQLDataTypes {
        match self {
            Some(val) => vec_u8_to_varchar(val.to_vec()),
            None => SQLDataTypes::NULL,
        }
    }
}

impl ToSQLData for Box<&[u8]> {
    fn to_sql_fmt(&self) -> SQLDataTypes {
        bytes_to_varchar(self)
    }
}
impl ToSQLData for Box<Vec<u8>> {
    fn to_sql_fmt(&self) -> SQLDataTypes {
        vec_u8_to_varchar(self.to_vec())
    }
}
impl ToSQLData for Option<Box<&[u8]>> {
    fn to_sql_fmt(&self) -> SQLDataTypes {
        match self {
            Some(val) => bytes_to_varchar(**val),
            None => SQLDataTypes::NULL,
        }
    }
}
impl ToSQLData for Option<Box<Vec<u8>>> {
    fn to_sql_fmt(&self) -> SQLDataTypes {
        match self {
            Some(val) => vec_u8_to_varchar(val.to_vec()),
            None => SQLDataTypes::NULL,
        }
    }
}

impl ToSQLData for usize {
    fn to_sql_fmt(&self) -> SQLDataTypes {
        let buff = *self as i64;
        SQLDataTypes::Number(buff)
    }
}
impl ToSQLData for Option<usize> {
    fn to_sql_fmt(&self) -> SQLDataTypes {
        let buff = if let Some(val) = *self {
            val as i64
        } else {
            return SQLDataTypes::NULL;
        };
        SQLDataTypes::Number(buff)
    }
}
impl ToSQLData for Box<usize> {
    fn to_sql_fmt(&self) -> SQLDataTypes {
        let buff = **self as i64;
        SQLDataTypes::Number(buff)
    }
}
impl ToSQLData for Option<Box<usize>> {
    fn to_sql_fmt(&self) -> SQLDataTypes {
        let buff = if let Some(val) = self {
            **val as i64
        } else {
            return SQLDataTypes::NULL;
        };
        SQLDataTypes::Number(buff)
    }
}

macro_rules! impl_fmt_data {
    ($data_type:ty, $enum_type:ident) => {
        impl ToSQLData for $data_type {
            fn to_sql_fmt(&self) -> SQLDataTypes {
                SQLDataTypes::$enum_type(self.to_owned().into())
            }
        }
    };
}
impl_fmt_data!(&str, Varchar);
impl_fmt_data!(String, Varchar);
impl_fmt_data!(&String, Varchar);
impl_fmt_data!(i8, Number);
impl_fmt_data!(i16, Number);
impl_fmt_data!(i32, Number);
impl_fmt_data!(i64, Number);
impl_fmt_data!(f32, Float);
impl_fmt_data!(f64, Float);
impl_fmt_data!(NaiveDateTime, Date);

macro_rules! impl_fmt_data_heap {
    ($data_type:ty, $enum_type:ident) => {
        impl ToSQLData for $data_type {
            fn to_sql_fmt(&self) -> SQLDataTypes {
                let buffer = *self.to_owned();
                SQLDataTypes::$enum_type(buffer.into())
            }
        }
    };
}
impl_fmt_data_heap!(Box<&str>, Varchar);
impl_fmt_data_heap!(Box<String>, Varchar);
impl_fmt_data_heap!(Box<&String>, Varchar);
impl_fmt_data_heap!(Box<i8>, Number);
impl_fmt_data_heap!(Box<i16>, Number);
impl_fmt_data_heap!(Box<i32>, Number);
impl_fmt_data_heap!(Box<i64>, Number);
impl_fmt_data_heap!(Box<f32>, Float);
impl_fmt_data_heap!(Box<f64>, Float);
impl_fmt_data_heap!(Box<NaiveDateTime>, Date);

macro_rules! impl_fmt_data_option {
    ($data_type:ty, $enum_type:ident) => {
        impl ToSQLData for $data_type {
            fn to_sql_fmt(&self) -> SQLDataTypes {
                match self {
                    Some(val) => SQLDataTypes::$enum_type(val.to_owned().into()),
                    None => SQLDataTypes::NULL,
                }
            }
        }
    };
}
impl_fmt_data_option!(Option<&str>, Varchar);
impl_fmt_data_option!(Option<String>, Varchar);
impl_fmt_data_option!(Option<&String>, Varchar);
impl_fmt_data_option!(Option<i8>, Number);
impl_fmt_data_option!(Option<i16>, Number);
impl_fmt_data_option!(Option<i32>, Number);
impl_fmt_data_option!(Option<i64>, Number);
impl_fmt_data_option!(Option<f32>, Float);
impl_fmt_data_option!(Option<f64>, Float);
impl_fmt_data_option!(Option<NaiveDateTime>, Date);

macro_rules! impl_fmt_data_option_heap {
    ($data_type:ty, $enum_type:ident) => {
        impl ToSQLData for $data_type {
            fn to_sql_fmt(&self) -> SQLDataTypes {
                match self {
                    Some(val) => {
                        let buffer = (*val.to_owned()).into();
                        SQLDataTypes::$enum_type(buffer)
                    }
                    None => SQLDataTypes::NULL,
                }
            }
        }
    };
}
impl_fmt_data_option_heap!(Option<Box<&str>>, Varchar);
impl_fmt_data_option_heap!(Option<Box<String>>, Varchar);
impl_fmt_data_option_heap!(Option<Box<&String>>, Varchar);
impl_fmt_data_option_heap!(Option<Box<i8>>, Number);
impl_fmt_data_option_heap!(Option<Box<i16>>, Number);
impl_fmt_data_option_heap!(Option<Box<i32>>, Number);
impl_fmt_data_option_heap!(Option<Box<i64>>, Number);
impl_fmt_data_option_heap!(Option<Box<f32>>, Float);
impl_fmt_data_option_heap!(Option<Box<f64>>, Float);
impl_fmt_data_option_heap!(Option<Box<NaiveDateTime>>, Date);
