use std::env;

fn main() {
    match run(&env::args().skip(1).collect::<Vec<_>>()) {
        Ok(output) => println!("{output}"),
        Err(error) => eprintln!("Error: {error}"),
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Date {
    year: i32,
    month: u32,
    day: u32,
}

impl Date {
    fn parse(value: &str) -> Result<Self, String> {
        let parts = value.split('-').collect::<Vec<_>>();
        if parts.len() != 3 {
            return Err(format!("invalid date '{value}', expected YYYY-MM-DD"));
        }
        let date = Self {
            year: parts[0]
                .parse()
                .map_err(|_| format!("invalid year in '{value}'"))?,
            month: parts[1]
                .parse()
                .map_err(|_| format!("invalid month in '{value}'"))?,
            day: parts[2]
                .parse()
                .map_err(|_| format!("invalid day in '{value}'"))?,
        };
        if date.month == 0 || date.month > 12 || date.day == 0 || date.day > date.days_in_month() {
            return Err(format!("date '{value}' is out of range"));
        }
        Ok(date)
    }

    fn is_leap_year(year: i32) -> bool {
        year % 4 == 0 && (year % 100 != 0 || year % 400 == 0)
    }

    fn days_in_month(self) -> u32 {
        match self.month {
            2 if Self::is_leap_year(self.year) => 29,
            2 => 28,
            4 | 6 | 9 | 11 => 30,
            _ => 31,
        }
    }

    fn add_days(self, days: i64) -> Self {
        let ordinal = days_from_civil(self.year, self.month, self.day) + days;
        let (year, month, day) = civil_from_days(ordinal);
        Self { year, month, day }
    }

    fn format(self) -> String {
        format!("{:04}-{:02}-{:02}", self.year, self.month, self.day)
    }
}

fn run(arguments: &[String]) -> Result<String, String> {
    match arguments {
        [date, amount] => {
            let date = Date::parse(date)?;
            let days = parse_duration(amount)?;
            Ok(date.add_days(days).format())
        }
        [first, second, flag] if flag == "--diff" => {
            let first = Date::parse(first)?;
            let second = Date::parse(second)?;
            Ok((days_from_civil(second.year, second.month, second.day)
                - days_from_civil(first.year, first.month, first.day))
            .to_string())
        }
        _ => Err("usage: date_calculator DATE (+2w|-3d|10) or DATE1 DATE2 --diff".to_string()),
    }
}

fn parse_duration(value: &str) -> Result<i64, String> {
    let (number, unit) = value.split_at(value.len().saturating_sub(1));
    let multiplier = match unit {
        "d" => 1,
        "w" => 7,
        _ if value
            .chars()
            .all(|character| character == '-' || character.is_ascii_digit()) =>
        {
            return value
                .parse::<i64>()
                .map_err(|_| format!("invalid day count '{value}'"));
        }
        _ => return Err(format!("invalid duration '{value}', use days or weeks")),
    };
    number
        .parse::<i64>()
        .map(|amount| amount * multiplier)
        .map_err(|_| format!("invalid duration '{value}'"))
}

// Howard Hinnant's proleptic Gregorian civil-date conversion, using 1970-01-01 as day 0.
fn days_from_civil(year: i32, month: u32, day: u32) -> i64 {
    let year = i64::from(year) - i64::from(month <= 2);
    let era = (if year >= 0 { year } else { year - 399 }) / 400;
    let year_of_era = year - era * 400;
    let month = i64::from(month);
    let day_of_year = (153 * (month + if month > 2 { -3 } else { 9 }) + 2) / 5 + i64::from(day) - 1;
    let day_of_era = year_of_era * 365 + year_of_era / 4 - year_of_era / 100 + day_of_year;
    era * 146_097 + day_of_era - 719_468
}

fn civil_from_days(days: i64) -> (i32, u32, u32) {
    let days = days + 719_468;
    let era = (if days >= 0 { days } else { days - 146_096 }) / 146_097;
    let day_of_era = days - era * 146_097;
    let year_of_era =
        (day_of_era - day_of_era / 1_460 + day_of_era / 36_524 - day_of_era / 146_096) / 365;
    let year = year_of_era + era * 400;
    let day_of_year = day_of_era - (365 * year_of_era + year_of_era / 4 - year_of_era / 100);
    let month_part = (5 * day_of_year + 2) / 153;
    let day = day_of_year - (153 * month_part + 2) / 5 + 1;
    let month = month_part + if month_part < 10 { 3 } else { -9 };
    let year = year + i64::from(month <= 2);
    (year as i32, month as u32, day as u32)
}

#[cfg(test)]
mod tests {
    use super::{Date, parse_duration, run};

    #[test]
    fn handles_month_and_leap_boundaries() {
        assert_eq!(
            run(&["2024-01-31".into(), "+1d".into()]).unwrap(),
            "2024-02-01"
        );
        assert_eq!(
            run(&["2024-02-28".into(), "+1d".into()]).unwrap(),
            "2024-02-29"
        );
        assert_eq!(
            run(&["2024-03-01".into(), "-1d".into()]).unwrap(),
            "2024-02-29"
        );
    }

    #[test]
    fn calculates_signed_differences() {
        assert_eq!(
            run(&["2024-01-01".into(), "2024-01-08".into(), "--diff".into()]).unwrap(),
            "7"
        );
        assert_eq!(
            run(&["2024-01-08".into(), "2024-01-01".into(), "--diff".into()]).unwrap(),
            "-7"
        );
    }

    #[test]
    fn parses_days_and_weeks() {
        assert_eq!(parse_duration("10").unwrap(), 10);
        assert_eq!(parse_duration("-2w").unwrap(), -14);
    }

    #[test]
    fn rejects_invalid_dates() {
        assert!(Date::parse("2023-02-29").is_err());
        assert!(Date::parse("2024-13-01").is_err());
    }
}
