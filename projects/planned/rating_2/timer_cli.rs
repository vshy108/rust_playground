// Solution flow:
// 1. Parse a positive duration such as `30s`, `5m`, or `1h`, or select stopwatch mode.
// 2. Keep duration parsing and clock formatting in pure helpers that are easy to test.
// 3. For a countdown, redraw the remaining time once per second and announce completion.
// 4. For a stopwatch, measure elapsed time until the user presses Enter.
use std::{
    env,
    io::{self, Write},
    thread,
    time::{Duration, Instant},
};

fn main() {
    if let Err(error) = run(&env::args().skip(1).collect::<Vec<_>>()) {
        eprintln!("Error: {error}");
    }
}

fn run(arguments: &[String]) -> Result<(), String> {
    let (command, remaining) = arguments
        .split_first()
        .ok_or_else(|| "usage: timer_cli <DURATION> [--sound] | stopwatch".to_string())?;

    if command == "stopwatch" {
        if !remaining.is_empty() {
            return Err("stopwatch mode does not accept additional arguments".to_string());
        }
        return run_stopwatch();
    }

    let duration = parse_duration(command)?;
    let sound = match remaining {
        [] => false,
        [flag] if flag == "--sound" => true,
        [value, ..] => return Err(format!("unknown option '{value}'")),
    };
    run_countdown(duration, sound)
}

fn parse_duration(input: &str) -> Result<Duration, String> {
    let (number, multiplier) = match input.chars().last() {
        Some('s') => (&input[..input.len() - 1], 1),
        Some('m') => (&input[..input.len() - 1], 60),
        Some('h') => (&input[..input.len() - 1], 60 * 60),
        Some(character) if character.is_ascii_digit() => (input, 1),
        _ => {
            return Err(format!(
                "'{input}' must be a duration such as 30s, 5m, or 1h"
            ))
        }
    };
    let amount = number
        .parse::<u64>()
        .map_err(|_| format!("'{input}' must start with a whole number"))?;
    if amount == 0 {
        return Err("duration must be greater than zero".to_string());
    }

    amount
        .checked_mul(multiplier)
        .map(Duration::from_secs)
        .ok_or_else(|| "duration is too large".to_string())
}

fn run_countdown(duration: Duration, sound: bool) -> Result<(), String> {
    let mut stdout = io::stdout().lock();
    for remaining_seconds in (1..=duration.as_secs()).rev() {
        write!(
            stdout,
            "\rRemaining: {}",
            format_duration(Duration::from_secs(remaining_seconds))
        )
        .map_err(|error| format!("failed to write countdown: {error}"))?;
        stdout
            .flush()
            .map_err(|error| format!("failed to flush countdown: {error}"))?;
        thread::sleep(Duration::from_secs(1));
    }

    writeln!(stdout, "\rTime's up!             ")
        .map_err(|error| format!("failed to write completion message: {error}"))?;
    if sound {
        write!(stdout, "\x07")
            .map_err(|error| format!("failed to play completion bell: {error}"))?;
    }
    Ok(())
}

fn run_stopwatch() -> Result<(), String> {
    println!("Stopwatch started. Press Enter to stop.");
    let started_at = Instant::now();
    let mut line = String::new();
    io::stdin()
        .read_line(&mut line)
        .map_err(|error| format!("failed to read standard input: {error}"))?;
    println!("Elapsed: {}", format_duration(started_at.elapsed()));
    Ok(())
}

fn format_duration(duration: Duration) -> String {
    let total_seconds = duration.as_secs();
    let hours = total_seconds / 3_600;
    let minutes = total_seconds % 3_600 / 60;
    let seconds = total_seconds % 60;

    if hours > 0 {
        format!("{hours}:{minutes:02}:{seconds:02}")
    } else {
        format!("{minutes:02}:{seconds:02}")
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use super::{format_duration, parse_duration};

    #[test]
    fn parses_supported_duration_units() {
        assert_eq!(parse_duration("45").unwrap(), Duration::from_secs(45));
        assert_eq!(parse_duration("2m").unwrap(), Duration::from_secs(120));
        assert_eq!(parse_duration("1h").unwrap(), Duration::from_secs(3_600));
    }

    #[test]
    fn rejects_zero_and_invalid_durations() {
        assert!(parse_duration("0s").is_err());
        assert!(parse_duration("2d").is_err());
        assert!(parse_duration("later").is_err());
    }

    #[test]
    fn formats_short_and_long_elapsed_times() {
        assert_eq!(format_duration(Duration::from_secs(65)), "01:05");
        assert_eq!(format_duration(Duration::from_secs(3_661)), "1:01:01");
    }
}
