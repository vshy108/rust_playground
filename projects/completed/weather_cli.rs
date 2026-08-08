use serde::Deserialize;
use std::env;

#[tokio::main]
async fn main() {
    match run(&env::args().skip(1).collect::<Vec<_>>()).await {
        Ok(output) => print!("{output}"),
        Err(error) => eprintln!("Error: {error}"),
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Unit {
    Celsius,
    Fahrenheit,
}

struct Options {
    latitude: f64,
    longitude: f64,
    unit: Unit,
    compact: bool,
}

#[derive(Debug, Deserialize)]
struct WeatherReport {
    current: CurrentConditions,
    daily: DailyForecast,
}

#[derive(Debug, Deserialize)]
struct CurrentConditions {
    temperature_2m: f64,
    wind_speed_10m: f64,
    weather_code: u8,
}

#[derive(Debug, Deserialize)]
struct DailyForecast {
    time: Vec<String>,
    weather_code: Vec<u8>,
    temperature_2m_max: Vec<f64>,
    temperature_2m_min: Vec<f64>,
}

async fn run(arguments: &[String]) -> Result<String, String> {
    let options = parse_options(arguments)?;
    let report = fetch_weather(&options).await?;
    Ok(format_report(&report, &options))
}

fn parse_options(arguments: &[String]) -> Result<Options, String> {
    let location = arguments
        .first()
        .ok_or_else(|| "usage: weather_cli LAT,LON [--fahrenheit] [--compact]".to_string())?;
    let (latitude, longitude) = location
        .split_once(',')
        .ok_or_else(|| "location must use LAT,LON".to_string())?;
    let latitude = latitude
        .parse::<f64>()
        .map_err(|_| "latitude must be a number".to_string())?;
    let longitude = longitude
        .parse::<f64>()
        .map_err(|_| "longitude must be a number".to_string())?;
    if !(-90.0..=90.0).contains(&latitude) || !(-180.0..=180.0).contains(&longitude) {
        return Err("location is outside valid latitude/longitude ranges".to_string());
    }
    let mut unit = Unit::Celsius;
    let mut compact = false;
    for argument in &arguments[1..] {
        match argument.as_str() {
            "--celsius" => unit = Unit::Celsius,
            "--fahrenheit" => unit = Unit::Fahrenheit,
            "--compact" => compact = true,
            value => return Err(format!("unknown option '{value}'")),
        }
    }
    Ok(Options {
        latitude,
        longitude,
        unit,
        compact,
    })
}

async fn fetch_weather(options: &Options) -> Result<WeatherReport, String> {
    let url = format!(
        "https://api.open-meteo.com/v1/forecast?latitude={}&longitude={}&current=temperature_2m,wind_speed_10m,weather_code&daily=weather_code,temperature_2m_max,temperature_2m_min&forecast_days=3&timezone=auto",
        options.latitude, options.longitude
    );
    reqwest::get(url)
        .await
        .map_err(|error| format!("weather request failed: {error}"))?
        .error_for_status()
        .map_err(|error| format!("weather service returned an error: {error}"))?
        .json::<WeatherReport>()
        .await
        .map_err(|error| format!("weather response was malformed: {error}"))
}

fn format_report(report: &WeatherReport, options: &Options) -> String {
    let temperature = format_temperature(report.current.temperature_2m, options.unit);
    let wind = format_wind(report.current.wind_speed_10m, options.unit);
    if options.compact {
        return format!(
            "{} {} | {} | wind {}\n",
            icon(report.current.weather_code),
            temperature,
            condition(report.current.weather_code),
            wind
        );
    }
    let mut output = format!(
        "Current: {} {} {}\nWind: {}\nForecast:\n",
        icon(report.current.weather_code),
        temperature,
        condition(report.current.weather_code),
        wind
    );
    for index in 0..report.daily.time.len().min(3) {
        output.push_str(&format!(
            "{} {} {}–{}\n",
            report.daily.time[index],
            icon(report.daily.weather_code[index]),
            format_temperature(report.daily.temperature_2m_min[index], options.unit),
            format_temperature(report.daily.temperature_2m_max[index], options.unit)
        ));
    }
    output
}

fn format_temperature(value: f64, unit: Unit) -> String {
    match unit {
        Unit::Celsius => format!("{value:.1}°C"),
        Unit::Fahrenheit => format!("{:.1}°F", value * 9.0 / 5.0 + 32.0),
    }
}

fn format_wind(value: f64, unit: Unit) -> String {
    match unit {
        Unit::Celsius => format!("{value:.1} km/h"),
        Unit::Fahrenheit => format!("{:.1} mph", value * 0.621_371),
    }
}

fn condition(code: u8) -> &'static str {
    match code {
        0 => "clear",
        1..=3 => "partly cloudy",
        45 | 48 => "fog",
        51..=67 => "rain",
        71..=77 => "snow",
        80..=99 => "showers",
        _ => "unknown",
    }
}

fn icon(code: u8) -> &'static str {
    match code {
        0 => "☀",
        1..=3 => "☁",
        45 | 48 => "≋",
        51..=67 | 80..=99 => "☂",
        71..=77 => "❄",
        _ => "?",
    }
}

#[cfg(test)]
mod tests {
    use super::{
        CurrentConditions, DailyForecast, Options, Unit, WeatherReport, format_report,
        parse_options,
    };

    fn fixture() -> WeatherReport {
        WeatherReport {
            current: CurrentConditions {
                temperature_2m: 25.0,
                wind_speed_10m: 10.0,
                weather_code: 1,
            },
            daily: DailyForecast {
                time: vec!["2026-08-09".to_string()],
                weather_code: vec![1],
                temperature_2m_max: vec![30.0],
                temperature_2m_min: vec![22.0],
            },
        }
    }

    #[test]
    fn parses_location_and_units() {
        let options = parse_options(&[
            "3.14,101.69".to_string(),
            "--fahrenheit".to_string(),
            "--compact".to_string(),
        ])
        .unwrap();
        assert_eq!(options.unit, Unit::Fahrenheit);
        assert!(options.compact);
    }

    #[test]
    fn formats_readable_forecast() {
        let options = Options {
            latitude: 0.0,
            longitude: 0.0,
            unit: Unit::Celsius,
            compact: false,
        };
        let output = format_report(&fixture(), &options);
        assert!(output.contains("Current: ☁ 25.0°C partly cloudy"));
        assert!(output.contains("2026-08-09 ☁ 22.0°C–30.0°C"));
    }

    #[test]
    fn formats_compact_fahrenheit_output() {
        let options = Options {
            latitude: 0.0,
            longitude: 0.0,
            unit: Unit::Fahrenheit,
            compact: true,
        };
        assert_eq!(
            format_report(&fixture(), &options),
            "☁ 77.0°F | partly cloudy | wind 6.2 mph\n"
        );
    }
}
