use std::{env, process::Command};
use time::{OffsetDateTime, Weekday};

#[derive(Debug, PartialEq, Eq)]
struct Schedule {
    minute: Field,
    hour: Field,
    day: Field,
    month: Field,
    weekday: Field,
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum Field {
    Any,
    Exact(u32),
    Step(u32),
}

fn main() {
    match run(&env::args().skip(1).collect::<Vec<_>>()) {
        Ok(output) => println!("{output}"),
        Err(error) => eprintln!("Error: {error}"),
    }
}

fn run(args: &[String]) -> Result<String, String> {
    let schedule = Schedule::parse(args.first().ok_or_else(|| {
        "usage: cron_scheduler \"MIN HOUR DOM MON DOW\" BASE_UNIX [--run COMMAND]".to_string()
    })?)?;
    let base = args
        .get(1)
        .ok_or_else(|| "BASE_UNIX is required".to_string())?
        .parse()
        .map_err(|_| "BASE_UNIX must be an integer".to_string())?;
    let next = schedule.next_after(base)?;
    if args.get(2).map(String::as_str) == Some("--run") {
        let command = args
            .get(3)
            .ok_or_else(|| "--run needs a command".to_string())?;
        Command::new("sh")
            .arg("-c")
            .arg(command)
            .status()
            .map_err(|error| error.to_string())?;
    }
    Ok(next.to_string())
}

impl Schedule {
    fn parse(expression: &str) -> Result<Self, String> {
        let fields = expression
            .split_whitespace()
            .map(Field::parse)
            .collect::<Result<Vec<_>, _>>()?;
        if fields.len() != 5 {
            return Err("schedule needs five fields: MIN HOUR DOM MON DOW".to_string());
        }
        Ok(Self {
            minute: fields[0].clone(),
            hour: fields[1].clone(),
            day: fields[2].clone(),
            month: fields[3].clone(),
            weekday: fields[4].clone(),
        })
    }

    fn next_after(&self, timestamp: i64) -> Result<i64, String> {
        let start =
            OffsetDateTime::from_unix_timestamp(timestamp).map_err(|error| error.to_string())?;
        for offset in 1..=5_260_800i64 {
            let candidate = start + time::Duration::minutes(offset);
            if self.matches(candidate) {
                return Ok(candidate.unix_timestamp());
            }
        }
        Err("no matching execution time within ten years".to_string())
    }

    fn matches(&self, value: OffsetDateTime) -> bool {
        field_matches(&self.minute, u32::from(value.minute()))
            && field_matches(&self.hour, u32::from(value.hour()))
            && field_matches(&self.day, u32::from(value.day()))
            && field_matches(&self.month, value.month() as u32)
            && field_matches(&self.weekday, weekday_number(value.weekday()))
    }
}

impl Field {
    fn parse(value: &str) -> Result<Self, String> {
        if value == "*" {
            return Ok(Self::Any);
        }
        if let Some(step) = value.strip_prefix("*/") {
            let step = step
                .parse()
                .map_err(|_| format!("invalid step '{value}'"))?;
            return if step > 0 {
                Ok(Self::Step(step))
            } else {
                Err("step must be positive".to_string())
            };
        }
        Ok(Self::Exact(
            value
                .parse()
                .map_err(|_| format!("invalid field '{value}'"))?,
        ))
    }
}

fn field_matches(field: &Field, value: u32) -> bool {
    match field {
        Field::Any => true,
        Field::Exact(expected) => *expected == value,
        Field::Step(step) => value.is_multiple_of(*step),
    }
}

fn weekday_number(day: Weekday) -> u32 {
    match day {
        Weekday::Sunday => 0,
        Weekday::Monday => 1,
        Weekday::Tuesday => 2,
        Weekday::Wednesday => 3,
        Weekday::Thursday => 4,
        Weekday::Friday => 5,
        Weekday::Saturday => 6,
    }
}

#[cfg(test)]
mod tests {
    use super::Schedule;
    use time::OffsetDateTime;

    #[test]
    fn computes_next_exact_minute() {
        let schedule = Schedule::parse("30 * * * *").unwrap();
        let base = OffsetDateTime::from_unix_timestamp(0)
            .unwrap()
            .unix_timestamp();
        assert_eq!(schedule.next_after(base).unwrap(), 30 * 60);
    }

    #[test]
    fn parses_steps_and_rejects_bad_shapes() {
        assert!(Schedule::parse("*/15 * * * *").is_ok());
        assert!(Schedule::parse("* * *").is_err());
        assert!(Schedule::parse("*/0 * * * *").is_err());
    }
}
