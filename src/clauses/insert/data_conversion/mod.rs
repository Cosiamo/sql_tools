use date::{date_conversion, date_w_abbrv_conversion};
use datetime::datetime_conversion;

use crate::data_types::SQLDataTypes;

pub mod dt_regex;
pub mod date;
pub mod datetime;

impl SQLDataTypes {
    pub fn format_data_types(&mut self) -> &mut SQLDataTypes {
        if let SQLDataTypes::Varchar(val) = self {
            if val.trim().len() == 0 {
                *self = SQLDataTypes::NULL;
            } else if !contains_number(val) {
                return self
            // If I want to convert percentages and currency to floats,
            // this is where I should add that functionality.
            // (Remove symbols)
            } else if val.trim().replace(".", "").chars().all(char::is_numeric) {
                if val.contains(".") {
                    // Add else here if data format == mm.dd.YYYY
                    if let Ok(float) = val.parse::<f64>() {
                        *self = SQLDataTypes::Float(float)
                    }
                } else {
                    if let Ok(int) = val.parse::<i64>() {
                        *self = SQLDataTypes::Number(int)
                    }
                }
            } else if contains_number(val) && is_dt(val) {
                if let Ok(dt) = datetime_conversion(val) {
                    *self = SQLDataTypes::Date(dt)
                } else {
                    if let Ok(date) = date_conversion(val) {
                        let dt = date
                            .and_hms_milli_opt(0, 0, 0, 0)
                            .unwrap();
                        *self = SQLDataTypes::Date(dt)
                    } else {
                        if let Ok(date) = date_w_abbrv_conversion(val) {
                            let dt = date
                                .and_hms_milli_opt(0, 0, 0, 0)
                                .unwrap();
                            *self = SQLDataTypes::Date(dt)
                        }
                    }
                }
            } 
        } 
        return self
    }
}

fn contains_number(input: &mut String) -> bool {
    let is_num = input.trim().chars().map(|char|{
        if char.is_ascii_digit() {
            return true;
        } else {
            return false;
        }
    }).collect::<Vec<bool>>();
    if is_num.contains(&true) {
        return true;
    } else {
        false
    }
}

fn is_dt(input: &mut String) -> bool {
    if input.contains("/")
    || input.contains("-")
    || input.contains(":") {
        return true
    } else {
        return false
    }
}