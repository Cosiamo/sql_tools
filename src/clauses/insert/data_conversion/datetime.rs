use chrono::NaiveDateTime;

pub fn datetime_conversion(input: &mut String) -> Result<NaiveDateTime, String> {
    let fmt_dt = input
        .replace("/", "-")
        .replace(".", "-")
        .trim()
        .to_ascii_uppercase();

    // mm/dd/yyyy hh:mm:ss AM
    if let Ok(date_time) = NaiveDateTime::parse_from_str(&fmt_dt.to_ascii_uppercase(), "%m-%d-%Y %I:%M:%S %p") {
        return Ok(date_time)
    }
    // dd/mm/yyyy hh:mm:ss AM
    if let Ok(date_time) = NaiveDateTime::parse_from_str(&fmt_dt.to_ascii_uppercase(), "%d-%m-%Y %I:%M:%S %p") {
        return Ok(date_time)
    }
    // yyyy/mm/dd hh:mm:ss AM
    if let Ok(date_time) = NaiveDateTime::parse_from_str(&fmt_dt.to_ascii_uppercase(), "%Y-%m-%d %I:%M:%S %p") {
        return Ok(date_time)
    }
    // yyyy/dd/mm hh:mm:ss AM
    if let Ok(date_time) = NaiveDateTime::parse_from_str(&fmt_dt.to_ascii_uppercase(), "%Y-%d-%m %I:%M:%S %p") {
        return Ok(date_time)
    }
    // mm/dd/yyyy hh:mm:ss
    if let Ok(date_time) = NaiveDateTime::parse_from_str(&fmt_dt.to_ascii_uppercase(), "%m-%d-%Y %H:%M:%S") {
        return Ok(date_time)
    }
    // dd/mm/yyyy hh:mm:ss
    if let Ok(date_time) = NaiveDateTime::parse_from_str(&fmt_dt.to_ascii_uppercase(), "%d-%m-%Y %H:%M:%S") {
        return Ok(date_time)
    }
    // yyyy/mm/dd hh:mm:ss
    if let Ok(date_time) = NaiveDateTime::parse_from_str(&fmt_dt.to_ascii_uppercase(), "%Y-%m-%d %H:%M:%S") {
        return Ok(date_time)
    }
    // mm/dd/yyyy hh:mm AM
    if let Ok(date_time) = NaiveDateTime::parse_from_str(&fmt_dt.to_ascii_uppercase(), "%m-%d-%Y %I:%M %p") {
        return Ok(date_time)
    }
    // dd/mm/yyyy hh:mm AM
    if let Ok(date_time) = NaiveDateTime::parse_from_str(&fmt_dt.to_ascii_uppercase(), "%d-%m-%Y %I:%M %p") {
        return Ok(date_time)
    }
    // yyyy/mm/dd hh:mm AM
    if let Ok(date_time) = NaiveDateTime::parse_from_str(&fmt_dt.to_ascii_uppercase(), "%Y-%m-%d %I:%M %p") {
        return Ok(date_time)
    }
    // yyyy/dd/mm hh:mm AM
    if let Ok(date_time) = NaiveDateTime::parse_from_str(&fmt_dt.to_ascii_uppercase(), "%Y-%d-%m %I:%M %p") {
        return Ok(date_time)
    }
    // mm/dd/yy hh:mm:ss AM
    if let Ok(date_time) = NaiveDateTime::parse_from_str(&fmt_dt.to_ascii_uppercase(), "%m-%d-%y %I:%M:%S %p") {
        return Ok(date_time)
    }
    // dd/mm/yy hh:mm:ss AM
    if let Ok(date_time) = NaiveDateTime::parse_from_str(&fmt_dt.to_ascii_uppercase(), "%d-%m-%y %I:%M:%S %p") {
        return Ok(date_time)
    }
    // yy/mm/dd hh:mm:ss AM
    if let Ok(date_time) = NaiveDateTime::parse_from_str(&fmt_dt.to_ascii_uppercase(), "%y-%m-%d %I:%M:%S %p") {
        return Ok(date_time)
    }
    // yy/dd/mm hh:mm:ss AM
    if let Ok(date_time) = NaiveDateTime::parse_from_str(&fmt_dt.to_ascii_uppercase(), "%y-%d-%m %I:%M:%S %p") {
        return Ok(date_time)
    }
    // mm/dd/yy hh:mm:ss
    if let Ok(date_time) = NaiveDateTime::parse_from_str(&fmt_dt.to_ascii_uppercase(), "%m-%d-%y %H:%M:%S") {
        return Ok(date_time)
    }
    // dd/mm/yy hh:mm:ss
    if let Ok(date_time) = NaiveDateTime::parse_from_str(&fmt_dt.to_ascii_uppercase(), "%d-%m-%y %H:%M:%S") {
        return Ok(date_time)
    }
    // yy/mm/dd hh:mm:ss
    if let Ok(date_time) = NaiveDateTime::parse_from_str(&fmt_dt.to_ascii_uppercase(), "%y-%m-%d %H:%M:%S") {
        return Ok(date_time)
    }
    // mm/dd/yy hh:mm AM
    if let Ok(date_time) = NaiveDateTime::parse_from_str(&fmt_dt.to_ascii_uppercase(), "%m-%d-%y %I:%M %p") {
        return Ok(date_time)
    }
    // dd/mm/yy hh:mm AM
    if let Ok(date_time) = NaiveDateTime::parse_from_str(&fmt_dt.to_ascii_uppercase(), "%d-%m-%y %I:%M %p") {
        return Ok(date_time)
    }
    // yy/mm/dd hh:mm AM
    if let Ok(date_time) = NaiveDateTime::parse_from_str(&fmt_dt.to_ascii_uppercase(), "%y-%m-%d %I:%M %p") {
        return Ok(date_time)
    }
    // yy/dd/mm hh:mm AM
    if let Ok(date_time) = NaiveDateTime::parse_from_str(&fmt_dt.to_ascii_uppercase(), "%y-%d-%m %I:%M %p") {
        return Ok(date_time)
    }

    return Err(input.clone())
}