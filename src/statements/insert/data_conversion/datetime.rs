use chrono::NaiveDateTime;

const FORMATS: &[&str] = &[
    // 4-digit year, AM/PM with seconds
    "%m-%d-%Y %I:%M:%S %p",
    "%d-%m-%Y %I:%M:%S %p",
    "%Y-%m-%d %I:%M:%S %p",
    "%Y-%d-%m %I:%M:%S %p",
    // 4-digit year, 24h with seconds
    "%m-%d-%Y %H:%M:%S",
    "%d-%m-%Y %H:%M:%S",
    "%Y-%m-%d %H:%M:%S",
    // 4-digit year, AM/PM without seconds
    "%m-%d-%Y %I:%M %p",
    "%d-%m-%Y %I:%M %p",
    "%Y-%m-%d %I:%M %p",
    "%Y-%d-%m %I:%M %p",
    // 4-digit year, 24h without seconds
    "%m-%d-%Y %H:%M",
    "%d-%m-%Y %H:%M",
    "%Y-%m-%d %H:%M",
    "%Y-%d-%m %H:%M",
    // 2-digit year, AM/PM with seconds
    "%m-%d-%y %I:%M:%S %p",
    "%d-%m-%y %I:%M:%S %p",
    "%y-%m-%d %I:%M:%S %p",
    "%y-%d-%m %I:%M:%S %p",
    // 2-digit year, 24h with seconds
    "%m-%d-%y %H:%M:%S",
    "%d-%m-%y %H:%M:%S",
    "%y-%m-%d %H:%M:%S",
    // 2-digit year, AM/PM without seconds
    "%m-%d-%y %I:%M %p",
    "%d-%m-%y %I:%M %p",
    "%y-%m-%d %I:%M %p",
    "%y-%d-%m %I:%M %p",
];

pub fn datetime_conversion(input: &mut String) -> Result<NaiveDateTime, String> {
    let fmt_dt = input
        .replace("/", "-")
        .replace(".", "-")
        .trim()
        .to_ascii_uppercase();

    for fmt in FORMATS {
        if let Ok(date_time) = NaiveDateTime::parse_from_str(&fmt_dt, fmt) {
            return Ok(date_time);
        }
    }

    Err(input.clone())
}
