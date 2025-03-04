use regex::bytes::Regex;
use once_cell::sync::Lazy;

// is_date
pub static DATE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"^[0-3]?[0-9].[0-3]?[0-9].(?:[0-9]{2})?[0-9]{2}$").unwrap()
});
// is_datetime
pub static DT12: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"^[0-3]?[0-9].[0-3]?[0-9].(?:[0-9]{2})?[0-9]{2} ([1-9]|0[1-9]|1[0-2]):[0-5][0-9] ([AaPp][Mm])$").unwrap()
});
pub static DT12_W_S: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"^[0-3]?[0-9].[0-3]?[0-9].(?:[0-9]{2})?[0-9]{2} ([1-9]|0[1-9]|1[0-2]):[0-5][0-9]:[0-5][0-9] ([AaPp][Mm])$").unwrap()
});
pub static DT24: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"^[0-3]?[0-9].[0-3]?[0-9].(?:[0-9]{2})?[0-9]{2} ([01]?[0-9]|2[0-3]):[0-5][0-9]$").unwrap()
});
pub static DT24_W_S: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"^[0-3]?[0-9].[0-3]?[0-9].(?:[0-9]{2})?[0-9]{2} ([01]?[0-9]|2[0-3]):[0-5][0-9]:[0-5][0-9]$").unwrap()
});
// is_time_24h
pub static H24: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"^([01]?[0-9]|2[0-3]):[0-5][0-9]$").unwrap()
});
pub static H24_W_S: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"^([01]?[0-9]|2[0-3]):[0-5][0-9]:[0-5][0-9]$").unwrap()
});
// is_time_12h
pub static H12: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"^([1-9]|0[1-9]|1[0-2]):[0-5][0-9] ([AaPp][Mm])$").unwrap()
});
pub static H12_W_S: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"^([1-9]|0[1-9]|1[0-2]):[0-5][0-9]:[0-5][0-9] ([AaPp][Mm])$").unwrap()
});
// is_number
pub static NUM: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"^[-+]?\d*\.?\d*(\d+[eE][-+]?)?\d+$").unwrap()
});
// is_date_w_abbrv
pub static DATE_W_ABBRV: Lazy<regex::Regex> = Lazy::new(|| {
    regex::Regex::new(r"^\d{2}-(JAN|FEB|MAR|APR|MAY|JUN|JUL|AUG|SEP|OCT|NOV|DEC)-\d{4}$").unwrap()
});