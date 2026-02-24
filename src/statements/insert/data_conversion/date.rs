use chrono::NaiveDate;

fn try_formats(s: &str, fmts: &[&str]) -> Option<NaiveDate> {
    fmts.iter().find_map(|fmt| NaiveDate::parse_from_str(s, fmt).ok())
}

pub fn date_conversion(input: &mut String) -> Result<NaiveDate, String> {
    let string = input.replace("/", "-").replace(".", "-").trim().to_string();
    let bytes = string.as_bytes();
    let len = string.len();

    // Detect 4-digit year position to avoid chrono's %Y matching 2-digit year strings.
    let year_first = len >= 5 && bytes[4] == b'-' && bytes[..4].iter().all(|b| b.is_ascii_digit());
    let year_last = len >= 5 && bytes[len - 5] == b'-' && bytes[len - 4..].iter().all(|b| b.is_ascii_digit());

    let result = if year_first {
        try_formats(&string, &["%Y-%m-%d", "%Y-%d-%m"])
    } else if year_last {
        try_formats(&string, &["%m-%d-%Y", "%d-%m-%Y"])
    } else {
        // 2-digit year: try year-last first (more common), then year-first
        try_formats(&string, &["%m-%d-%y", "%d-%m-%y", "%y-%m-%d", "%y-%d-%m"])
    };

    result.ok_or_else(|| input.clone())
}

pub fn date_w_abbrv_conversion(input: &mut String) -> Result<NaiveDate, String> {
    let string = input.replace("/", "-").replace(".", "-").trim().to_string();
    let bytes = string.as_bytes();
    let len = string.len();

    let year_last_4 = len >= 5 && bytes[len - 5] == b'-' && bytes[len - 4..].iter().all(|b| b.is_ascii_digit());
    let result = if year_last_4 {
        try_formats(&string, &["%d-%b-%Y"])
    } else {
        try_formats(&string, &["%d-%b-%y"])
    };

    result.ok_or_else(|| input.clone())
}
