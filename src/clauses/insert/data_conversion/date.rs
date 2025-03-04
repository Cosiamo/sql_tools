use chrono::NaiveDate;

pub fn date_conversion(input: &mut String) -> Result<NaiveDate, String> {
    let string = &input
        .replace("/", "-")
        .replace(".", "-")
        .trim()
        .to_string();
    match string.len() {
        10 => match string {
            // mm/dd/yyyy
            string_to_date if &string_to_date[2..3] == "-" && &string_to_date[5..6] == "-"
            => match NaiveDate::parse_from_str(string_to_date, "%m-%d-%Y") {
                Ok(date) => return Ok(date),
                // dd/mm/yyyy
                Err(_) => match NaiveDate::parse_from_str(string_to_date, "%d-%m-%Y") {
                    Ok(date) => return Ok(date),
                    Err(_) => return Err(input.clone()),
                },
            },
            // yyyy/mm/dd
            string_to_date if &string_to_date[4..5] == "-" && &string_to_date[7..8] == "-"
            => match NaiveDate::parse_from_str(string_to_date, "%Y-%m-%d") {
                Ok(date) => return Ok(date),
                // yyyy/dd/mm
                Err(_) => match NaiveDate::parse_from_str(string_to_date, "%Y-%d-%m") {
                    Ok(date) => return Ok(date),
                    Err(_) => return Err(input.clone()),
                },
            },
            _ => return Err(input.clone()),
        },
        9 => match string {
            // m/dd/yyyy
            string_to_date if &string_to_date[1..2] == "-" && &string_to_date[4..5] == "-" 
            => match NaiveDate::parse_from_str(string_to_date, "%m-%d-%Y") {
                Ok(date) => return Ok(date),
                Err(_) => return Err(input.clone()),
            },
            // mm/d/yyyy
            string_to_date if &string_to_date[2..3] == "-" && &string_to_date[4..5] == "-" 
            => match NaiveDate::parse_from_str(string_to_date, "%m-%d-%Y") {
                Ok(date) => return Ok(date),
                // dd/m/yyyy
                Err(_) => match NaiveDate::parse_from_str(string_to_date, "%d-%m-%Y") {
                    Ok(date) => return Ok(date),
                    Err(_) => return Err(input.clone()),
                },
            },
            // yyyy/mm/d
            string_to_date if &string_to_date[4..5] == "-" && &string_to_date[7..8] == "-" 
            => match NaiveDate::parse_from_str(string_to_date, "%Y-%m-%d") {
                Ok(date) => return Ok(date),
                // yyyy/dd/m
                Err(_) => match NaiveDate::parse_from_str(string_to_date, "%Y-%d-%m") {
                    Ok(date) => return Ok(date),
                    Err(_) => return Err(input.clone()),
                },
            },
            // yyyy/m/dd
            string_to_date if &string_to_date[4..5] == "-" && &string_to_date[6..7] == "-" 
            => match NaiveDate::parse_from_str(string_to_date, "%Y-%m-%d") {
                Ok(date) => return Ok(date),
                Err(_) => return Err(input.clone()),
            },
            _ => return Err(input.clone()),
        },
        8 => match string {
            // m/d/yyyy
            string_to_date if &string_to_date[1..2] == "-" && &string_to_date[3..4] == "-" 
            => match NaiveDate::parse_from_str(string_to_date, "%m-%d-%Y") {
                Ok(date) => return Ok(date),
                Err(_) => return Err(input.clone()),
            },
            // yyyy/m/d
            string_to_date if &string_to_date[4..5] == "-" && &string_to_date[6..7] == "-" 
            => match NaiveDate::parse_from_str(string_to_date, "%Y-%m-%d") {
                Ok(date) => return Ok(date),
                Err(_) => return Err(input.clone()),
            },
            // mm/dd/yy
            string_to_date if &string_to_date[2..3] == "-" && &string_to_date[5..6] == "-"
            => match NaiveDate::parse_from_str(string_to_date, "%m-%d-%y") {
                Ok(date) => return Ok(date),
                // dd/mm/yy
                Err(_) => match NaiveDate::parse_from_str(string_to_date, "%d-%m-%y") {
                    Ok(date) => return Ok(date),
                    Err(_) => return Err(input.clone()),
                },
            },
            _ => return Err(input.clone()),
        },
        7 => match string {
            // m/dd/yy
            string_to_date if &string_to_date[1..2] == "-" && &string_to_date[4..5] == "-"
            => match NaiveDate::parse_from_str(string_to_date, "%m-%d-%y") {
                Ok(date) => return Ok(date),
                Err(_) => return Err(input.clone()),
            },
            // mm/d/yy
            string_to_date if &string_to_date[2..3] == "-" && &string_to_date[4..5] == "-"
            => match NaiveDate::parse_from_str(string_to_date, "%m-%d-%y") {
                Ok(date) => return Ok(date),
                // dd/m/yy
                Err(_) => match NaiveDate::parse_from_str(string_to_date, "%d-%m-%y") {
                    Ok(date) => return Ok(date),
                    // yy/m/dd
                    Err(_) => match NaiveDate::parse_from_str(string_to_date, "%y-%m-%d") {
                        Ok(date) => return Ok(date),
                        Err(_) => return Err(input.clone()),
                    },
                },
            },
            // yy/mm/d
            string_to_date if &string_to_date[2..3] == "-" && &string_to_date[5..6] == "-"
            => match NaiveDate::parse_from_str(string_to_date, "%y-%m-%d") {
                Ok(date) => return Ok(date),
                // yy/dd/m
                Err(_) => match NaiveDate::parse_from_str(string_to_date, "%y-%d-%m") {
                    Ok(date) => return Ok(date),
                    Err(_) => return Err(input.clone()),
                },
            },
            _ => return Err(input.clone()),
        },
        6 => match string {
            // m/d/yy
            string_to_date if &string_to_date[1..2] == "-" && &string_to_date[3..4] == "-"
            => match NaiveDate::parse_from_str(string_to_date, "%m-%d-%y") {
                Ok(date) => return Ok(date),
                Err(_) => return Err(input.clone()),
            },
            // yy/m/d
            string_to_date if &string_to_date[2..3] == "-" && &string_to_date[4..5] == "-"
            => match NaiveDate::parse_from_str(string_to_date, "%y-%m-%d") {
                Ok(date) => return Ok(date),
                Err(_) => return Err(input.clone()),
            },
            _ => return Err(input.clone()),
        },
        _ => return Err(input.clone()),
    }
}

pub fn date_w_abbrv_conversion(input: &mut String) -> Result<NaiveDate, String> {
    let string = &input.replace("/", "-").replace(".", "-").trim().to_string();
        match string.len() {
            11 => match string {
                // dd/mmm/yyyy
                string_to_date if &string_to_date[2..3] == "-" && &string_to_date[6..7] == "-" 
                => match NaiveDate::parse_from_str(string_to_date, "%d-%b-%Y") {
                    Ok(date) => return Ok(date),
                    Err(_) => return Err(input.clone()),
                },
                _ => return Err(input.clone()),
            },
            9 => match string {
                // dd/mmm/yy
                string_to_date if &string_to_date[2..3] == "-" && &string_to_date[6..7] == "-" 
                => match NaiveDate::parse_from_str(string_to_date, "%d-%b-%y") {
                    Ok(date) => return Ok(date),
                    Err(_) => return Err(input.clone()),
                },
                _ => return Err(input.clone()),
            },
            _ => return Err(input.clone()),
        }
}