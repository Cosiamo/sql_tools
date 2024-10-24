use chrono::NaiveDateTime;

use super::{ToSQLData, SQLDataTypes};


impl ToSQLData for SQLDataTypes { 
    fn fmt_data(self) -> Self { self } 
    fn fmt_data_borrowed(&self) -> SQLDataTypes { self.to_owned() }
}

impl ToSQLData for &[u8] {
    fn fmt_data(self) -> SQLDataTypes {
        let clone_on_write_string = String::from_utf8_lossy(self);
        let utf8_string = clone_on_write_string.replace(|c: char| !c.is_ascii(), "");
        SQLDataTypes::VARCHAR(utf8_string)
    }
    fn fmt_data_borrowed(&self) -> SQLDataTypes {
        let clone_on_write_string = String::from_utf8_lossy(self);
        let utf8_string = clone_on_write_string.replace(|c: char| !c.is_ascii(), "");
        SQLDataTypes::VARCHAR(utf8_string)
    }
}
impl ToSQLData for Vec<u8> {
    fn fmt_data(self) -> SQLDataTypes {
        let utf8_string = String::from_utf8(self)
            .map_err(|non_utf8| 
                String::from_utf8_lossy(non_utf8.as_bytes()).into_owned()
            )
            .unwrap();
        SQLDataTypes::VARCHAR(utf8_string)
    }
    fn fmt_data_borrowed(&self) -> SQLDataTypes {
        let utf8_string = String::from_utf8(self.to_vec())
            .map_err(|non_utf8| 
                String::from_utf8_lossy(non_utf8.as_bytes()).into_owned()
            )
            .unwrap();
        SQLDataTypes::VARCHAR(utf8_string)
    }
}
impl ToSQLData for Option<&[u8]> {
    fn fmt_data(self) -> SQLDataTypes {
        match self {
            Some(val) => {
                let clone_on_write_string = String::from_utf8_lossy(val);
                let utf8_string = clone_on_write_string.replace(|c: char| !c.is_ascii(), "");
                SQLDataTypes::VARCHAR(utf8_string)
            },
            None => SQLDataTypes::NULL,
        }
    }
    fn fmt_data_borrowed(&self) -> SQLDataTypes {
        match self {
            Some(val) => {
                let clone_on_write_string = String::from_utf8_lossy(val);
                let utf8_string = clone_on_write_string.replace(|c: char| !c.is_ascii(), "");
                SQLDataTypes::VARCHAR(utf8_string)
            },
            None => SQLDataTypes::NULL,
        }
    }
}
impl ToSQLData for Option<Vec<u8>> {
    fn fmt_data(self) -> SQLDataTypes {
        match self {
            Some(val) => {
                let utf8_string = String::from_utf8(val)
                    .map_err(|non_utf8| 
                        String::from_utf8_lossy(non_utf8.as_bytes()).into_owned()
                    )
                    .unwrap();
                SQLDataTypes::VARCHAR(utf8_string)
            },
            None => SQLDataTypes::NULL,
        }
    }
    fn fmt_data_borrowed(&self) -> SQLDataTypes {
        match self {
            Some(val) => {
                let utf8_string = String::from_utf8(val.to_vec())
                    .map_err(|non_utf8| 
                        String::from_utf8_lossy(non_utf8.as_bytes()).into_owned()
                    )
                    .unwrap();
                SQLDataTypes::VARCHAR(utf8_string)
            },
            None => SQLDataTypes::NULL,
        }
    }
}

macro_rules! impl_fmt_data {
    ($data_type:ty, $enum_type:ident) => {
        impl ToSQLData for $data_type {
            fn fmt_data(self) -> SQLDataTypes { SQLDataTypes::$enum_type(self.into()) }
            fn fmt_data_borrowed(&self) -> SQLDataTypes { SQLDataTypes::$enum_type(self.to_owned().into()) }
        }
    };
}
impl_fmt_data!(&str, VARCHAR);
impl_fmt_data!(String, VARCHAR);
impl_fmt_data!(i8, INT);
impl_fmt_data!(i16, INT);
impl_fmt_data!(i32, INT);
impl_fmt_data!(i64, INT);
impl_fmt_data!(f32, FLOAT);
impl_fmt_data!(f64, FLOAT);
impl_fmt_data!(NaiveDateTime, DATE);

macro_rules! impl_fmt_data_option {
    ($data_type:ty, $enum_type:ident) => {
        impl ToSQLData for $data_type {
            fn fmt_data(self) -> SQLDataTypes {
                match self {
                    Some(val) => SQLDataTypes::$enum_type(val.into()),
                    None => SQLDataTypes::NULL,
                }
            }
            fn fmt_data_borrowed(&self) -> SQLDataTypes {
                match self {
                    Some(val) => SQLDataTypes::$enum_type(val.to_owned().into()),
                    None => SQLDataTypes::NULL,
                }
            }
        }
    };
}
impl_fmt_data_option!(Option<&str>, VARCHAR);
impl_fmt_data_option!(Option<String>, VARCHAR);
impl_fmt_data_option!(Option<i8>, INT);
impl_fmt_data_option!(Option<i16>, INT);
impl_fmt_data_option!(Option<i32>, INT);
impl_fmt_data_option!(Option<i64>, INT);
impl_fmt_data_option!(Option<f32>, FLOAT);
impl_fmt_data_option!(Option<f64>, FLOAT);
impl_fmt_data_option!(Option<NaiveDateTime>, DATE);