use chrono::NaiveDateTime;

use super::{SQLDataTypes, ToSQLData};

impl ToSQLData for SQLDataTypes {
    fn fmt_data(&self) -> Self {
        self.to_owned()
    }
}
impl ToSQLData for Option<SQLDataTypes> {
    fn fmt_data(&self) -> SQLDataTypes {
        match self {
            Some(val) => val.to_owned(),
            None => SQLDataTypes::NULL,
        }
    }
}
impl ToSQLData for Box<SQLDataTypes> {
    fn fmt_data(&self) -> SQLDataTypes {
        *self.to_owned()
    }
}
impl ToSQLData for Option<Box<SQLDataTypes>> {
    fn fmt_data(&self) -> SQLDataTypes {
        match self {
            Some(val) => *val.to_owned(),
            None => SQLDataTypes::NULL,
        }
    }
}

impl ToSQLData for &[u8] {
    fn fmt_data(&self) -> SQLDataTypes {
        let clone_on_write_string = String::from_utf8_lossy(self);
        let utf8_string = clone_on_write_string.replace(|c: char| !c.is_ascii(), "");
        SQLDataTypes::Varchar(utf8_string)
    }
}
impl ToSQLData for Vec<u8> {
    fn fmt_data(&self) -> SQLDataTypes {
        let utf8_string = String::from_utf8(self.to_vec())
            .map_err(|non_utf8| String::from_utf8_lossy(non_utf8.as_bytes()).into_owned())
            .unwrap();
        SQLDataTypes::Varchar(utf8_string)
    }
}
impl ToSQLData for Option<&[u8]> {
    fn fmt_data(&self) -> SQLDataTypes {
        match self {
            Some(val) => {
                let clone_on_write_string = String::from_utf8_lossy(val);
                let utf8_string = clone_on_write_string.replace(|c: char| !c.is_ascii(), "");
                SQLDataTypes::Varchar(utf8_string)
            }
            None => SQLDataTypes::NULL,
        }
    }
}
impl ToSQLData for Option<Vec<u8>> {
    fn fmt_data(&self) -> SQLDataTypes {
        match self {
            Some(val) => {
                let utf8_string = String::from_utf8(val.to_vec())
                    .map_err(|non_utf8| String::from_utf8_lossy(non_utf8.as_bytes()).into_owned())
                    .unwrap();
                SQLDataTypes::Varchar(utf8_string)
            }
            None => SQLDataTypes::NULL,
        }
    }
}

impl ToSQLData for Box<&[u8]> {
    fn fmt_data(&self) -> SQLDataTypes {
        let clone_on_write_string = String::from_utf8_lossy(self);
        let utf8_string = clone_on_write_string.replace(|c: char| !c.is_ascii(), "");
        SQLDataTypes::Varchar(utf8_string)
    }
}
impl ToSQLData for Box<Vec<u8>> {
    fn fmt_data(&self) -> SQLDataTypes {
        let utf8_string = String::from_utf8(self.to_vec())
            .map_err(|non_utf8| String::from_utf8_lossy(non_utf8.as_bytes()).into_owned())
            .unwrap();
        SQLDataTypes::Varchar(utf8_string)
    }
}
impl ToSQLData for Option<Box<&[u8]>> {
    fn fmt_data(&self) -> SQLDataTypes {
        match &self {
            Some(val) => {
                let clone_on_write_string = String::from_utf8_lossy(**val);
                let utf8_string = clone_on_write_string.replace(|c: char| !c.is_ascii(), "");
                SQLDataTypes::Varchar(utf8_string)
            }
            None => SQLDataTypes::NULL,
        }
    }
}
impl ToSQLData for Option<Box<Vec<u8>>> {
    fn fmt_data(&self) -> SQLDataTypes {
        match &self {
            Some(val) => {
                let utf8_string = String::from_utf8(val.to_vec())
                    .map_err(|non_utf8| String::from_utf8_lossy(non_utf8.as_bytes()).into_owned())
                    .unwrap();
                SQLDataTypes::Varchar(utf8_string)
            }
            None => SQLDataTypes::NULL,
        }
    }
}

impl ToSQLData for usize {
    fn fmt_data(&self) -> SQLDataTypes {
        let buff = *self as i64;
        SQLDataTypes::Number(buff)
    }
}
impl ToSQLData for Option<usize> {
    fn fmt_data(&self) -> SQLDataTypes {
        let buff = if let Some(val) = *self {
            val as i64
        } else {
            return SQLDataTypes::NULL;
        };
        SQLDataTypes::Number(buff)
    }
}
impl ToSQLData for Box<usize> {
    fn fmt_data(&self) -> SQLDataTypes {
        let buff = **self as i64;
        SQLDataTypes::Number(buff)
    }
}
impl ToSQLData for Option<Box<usize>> {
    fn fmt_data(&self) -> SQLDataTypes {
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
            fn fmt_data(&self) -> SQLDataTypes {
                SQLDataTypes::$enum_type(self.to_owned().into())
            }
        }
    };
}
impl_fmt_data!(&str, Varchar);
impl_fmt_data!(String, Varchar);
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
            fn fmt_data(&self) -> SQLDataTypes {
                let buffer = *self.to_owned();
                SQLDataTypes::$enum_type(buffer.into())
            }
        }
    };
}
impl_fmt_data_heap!(Box<&str>, Varchar);
impl_fmt_data_heap!(Box<String>, Varchar);
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
            fn fmt_data(&self) -> SQLDataTypes {
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
            fn fmt_data(&self) -> SQLDataTypes {
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
impl_fmt_data_option_heap!(Option<Box<i8>>, Number);
impl_fmt_data_option_heap!(Option<Box<i16>>, Number);
impl_fmt_data_option_heap!(Option<Box<i32>>, Number);
impl_fmt_data_option_heap!(Option<Box<i64>>, Number);
impl_fmt_data_option_heap!(Option<Box<f32>>, Float);
impl_fmt_data_option_heap!(Option<Box<f64>>, Float);
impl_fmt_data_option_heap!(Option<Box<NaiveDateTime>>, Date);
