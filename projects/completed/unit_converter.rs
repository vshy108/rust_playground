// Solution flow:
// 1. Parse a numeric value, source unit, and target unit from the command line.
// 2. Map unit aliases to a category and a factor relative to that category's base unit.
// 3. Reject conversions across unrelated categories before doing any arithmetic.
// 4. Convert through the base unit and print a normalized, trailing-zero-free result.
use std::env;

fn main() {
    match run(&env::args().skip(1).collect::<Vec<_>>()) {
        Ok(output) => println!("{output}"),
        Err(error) => eprintln!("Error: {error}"),
    }
}

fn run(arguments: &[String]) -> Result<String, String> {
    let [value, source, target] = arguments else {
        return Err("usage: unit_converter <value> <source-unit> <target-unit>".to_string());
    };
    let value = value
        .parse::<f64>()
        .map_err(|_| format!("'{value}' is not a number"))?;
    let converted = convert(value, source, target)?;
    Ok(format!("{} {}", format_number(converted), target))
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Category {
    Length,
    Mass,
    Temperature,
    Speed,
}

#[derive(Clone, Copy)]
struct Unit {
    category: Category,
    factor: f64,
}

fn parse_unit(input: &str) -> Result<Unit, String> {
    let unit = match input.to_ascii_lowercase().as_str() {
        "m" | "meter" | "meters" => Unit {
            category: Category::Length,
            factor: 1.0,
        },
        "km" | "kilometer" | "kilometers" => Unit {
            category: Category::Length,
            factor: 1_000.0,
        },
        "cm" | "centimeter" | "centimeters" => Unit {
            category: Category::Length,
            factor: 0.01,
        },
        "mi" | "mile" | "miles" => Unit {
            category: Category::Length,
            factor: 1_609.344,
        },
        "kg" | "kilogram" | "kilograms" => Unit {
            category: Category::Mass,
            factor: 1.0,
        },
        "g" | "gram" | "grams" => Unit {
            category: Category::Mass,
            factor: 0.001,
        },
        "lb" | "lbs" | "pound" | "pounds" => Unit {
            category: Category::Mass,
            factor: 0.453_592_37,
        },
        "c" | "celsius" => Unit {
            category: Category::Temperature,
            factor: 1.0,
        },
        "f" | "fahrenheit" => Unit {
            category: Category::Temperature,
            factor: 1.0,
        },
        "k" | "kelvin" => Unit {
            category: Category::Temperature,
            factor: 1.0,
        },
        "m/s" => Unit {
            category: Category::Speed,
            factor: 1.0,
        },
        "km/h" | "kph" => Unit {
            category: Category::Speed,
            factor: 1.0 / 3.6,
        },
        _ => return Err(format!("unsupported unit '{input}'")),
    };
    Ok(unit)
}

fn convert(value: f64, source_name: &str, target_name: &str) -> Result<f64, String> {
    let source = parse_unit(source_name)?;
    let target = parse_unit(target_name)?;
    if source.category != target.category {
        return Err(format!("cannot convert {source_name} to {target_name}"));
    }

    if source.category == Category::Temperature {
        return convert_temperature(value, source_name, target_name);
    }

    Ok(value * source.factor / target.factor)
}

fn convert_temperature(value: f64, source: &str, target: &str) -> Result<f64, String> {
    let celsius = match source.to_ascii_lowercase().as_str() {
        "c" | "celsius" => value,
        "f" | "fahrenheit" => (value - 32.0) * 5.0 / 9.0,
        "k" | "kelvin" => value - 273.15,
        _ => return Err(format!("unsupported temperature unit '{source}'")),
    };
    Ok(match target.to_ascii_lowercase().as_str() {
        "c" | "celsius" => celsius,
        "f" | "fahrenheit" => celsius * 9.0 / 5.0 + 32.0,
        "k" | "kelvin" => celsius + 273.15,
        _ => return Err(format!("unsupported temperature unit '{target}'")),
    })
}

fn format_number(value: f64) -> String {
    let mut formatted = format!("{value:.6}");
    while formatted.contains('.') && formatted.ends_with('0') {
        formatted.pop();
    }
    formatted.trim_end_matches('.').to_string()
}

#[cfg(test)]
mod tests {
    use super::{convert, format_number};

    #[test]
    fn converts_length_mass_and_temperature() {
        assert_eq!(convert(2.0, "km", "m"), Ok(2_000.0));
        assert_eq!(convert(1_000.0, "g", "kg"), Ok(1.0));
        assert_eq!(convert(32.0, "f", "c"), Ok(0.0));
    }

    #[test]
    fn converts_compound_speed_units() {
        let meters_per_second = convert(36.0, "km/h", "m/s").unwrap();
        assert!((meters_per_second - 10.0).abs() < f64::EPSILON);
    }

    #[test]
    fn rejects_unsupported_and_incompatible_units() {
        assert!(convert(1.0, "m", "kg").is_err());
        assert!(convert(1.0, "lightyear", "m").is_err());
    }

    #[test]
    fn formats_with_normalized_precision() {
        assert_eq!(format_number(12.5), "12.5");
        assert_eq!(format_number(2.0), "2");
        assert_eq!(format_number(1.234_567_8), "1.234568");
    }
}
