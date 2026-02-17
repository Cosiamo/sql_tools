use date::{date_conversion, date_w_abbrv_conversion};
use datetime::datetime_conversion;

use crate::data_types::SQLDataTypes;

pub mod date;
pub mod datetime;
pub mod dt_regex;

impl SQLDataTypes {
    pub fn format_data_types(&mut self) -> &mut SQLDataTypes {
        let val = if let SQLDataTypes::Varchar(val) = self {
            val
        } else {
            return self;
        };
        if val.trim().is_empty() {
            *self = SQLDataTypes::NULL;
            return self;
        }
        if !contains_number(val) {
            return self;
        }
        if let Ok(int) = val.parse::<i64>() {
            *self = SQLDataTypes::Number(int);
            return self;
        }
        if let Ok(float) = val.parse::<f64>() {
            *self = SQLDataTypes::Float(float);
            return self;
        }
        if contains_number(val) && is_dt(val) {
            if let Some(val) = date_match(val) {
                *self = val;
                return self;
            }
        }
        self
    }
}

fn contains_number(input: &str) -> bool {
    input.trim().chars().any(|character| character.is_ascii_digit())
}

fn is_dt(input: &str) -> bool {
    input.contains("/") || input.contains("-") || input.contains(":")
}

fn date_match(val: &mut String) -> Option<SQLDataTypes> {
    if let Ok(dt) = datetime_conversion(val) {
        return Some(SQLDataTypes::Date(dt));
    }
    if let Ok(date) = date_conversion(val) {
        let dt = date.and_hms_milli_opt(0, 0, 0, 0).unwrap();
        return Some(SQLDataTypes::Date(dt));
    }
    if let Ok(date) = date_w_abbrv_conversion(val) {
        let dt = date.and_hms_milli_opt(0, 0, 0, 0).unwrap();
        return Some(SQLDataTypes::Date(dt));
    }
    None
}
