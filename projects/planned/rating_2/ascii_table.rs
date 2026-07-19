// Solution flow:
// 1. Read an optional numeric range or `--latin1` flag from the command line.
// 2. Turn every number in that range into a row containing decimal, hexadecimal, and text forms.
// 3. Replace control characters with readable labels instead of printing terminal control codes.
// 4. Render the rows into one predictable table, then print it.
use std::{env, ops::RangeInclusive};

const ASCII_END: u16 = 127;
const LATIN1_END: u16 = 255;

fn main() {
    match run(&env::args().skip(1).collect::<Vec<_>>()) {
        Ok(table) => print!("{table}"),
        Err(error) => eprintln!("Error: {error}"),
    }
}

fn run(arguments: &[String]) -> Result<String, String> {
    let range = parse_range_argument(arguments)?;
    Ok(render_table(&build_rows(range)))
}

fn parse_range_argument(arguments: &[String]) -> Result<RangeInclusive<u16>, String> {
    let mut use_latin1 = false;
    let mut selected_range = None;
    let mut index = 0;

    while index < arguments.len() {
        match arguments[index].as_str() {
            "--latin1" => use_latin1 = true,
            "--range" => {
                index += 1;
                let value = arguments
                    .get(index)
                    .ok_or_else(|| "--range needs a value such as 32-126".to_string())?;
                selected_range = Some(parse_range(value)?);
            }
            value if !value.starts_with('-') && selected_range.is_none() => {
                selected_range = Some(parse_range(value)?);
            }
            value => return Err(format!("unknown option '{value}'")),
        }

        index += 1;
    }

    Ok(selected_range.unwrap_or(0..=if use_latin1 { LATIN1_END } else { ASCII_END }))
}

fn parse_range(value: &str) -> Result<RangeInclusive<u16>, String> {
    let (start, end) = value
        .split_once('-')
        .ok_or_else(|| format!("range '{value}' must use START-END"))?;
    let start = parse_code(start)?;
    let end = parse_code(end)?;

    if start > end {
        return Err("range start must not be greater than its end".to_string());
    }
    if end > LATIN1_END {
        return Err(format!("range values must be between 0 and {LATIN1_END}"));
    }

    Ok(start..=end)
}

fn parse_code(value: &str) -> Result<u16, String> {
    let value = value.trim();
    let parsed = match value
        .strip_prefix("0x")
        .or_else(|| value.strip_prefix("0X"))
    {
        Some(hexadecimal) => u16::from_str_radix(hexadecimal, 16),
        None => value.parse(),
    };

    parsed.map_err(|_| format!("'{value}' is not a valid character code"))
}

#[derive(Debug, PartialEq, Eq)]
struct AsciiRow {
    decimal: u16,
    hexadecimal: String,
    character: String,
}

fn build_rows(range: RangeInclusive<u16>) -> Vec<AsciiRow> {
    range
        .map(|code| AsciiRow {
            decimal: code,
            hexadecimal: format!("0x{code:02X}"),
            character: character_label(code),
        })
        .collect()
}

fn character_label(code: u16) -> String {
    let label = match code {
        0 => "NUL",
        1 => "SOH",
        2 => "STX",
        3 => "ETX",
        4 => "EOT",
        5 => "ENQ",
        6 => "ACK",
        7 => "BEL",
        8 => "BS",
        9 => "TAB",
        10 => "LF",
        11 => "VT",
        12 => "FF",
        13 => "CR",
        14 => "SO",
        15 => "SI",
        16 => "DLE",
        17 => "DC1",
        18 => "DC2",
        19 => "DC3",
        20 => "DC4",
        21 => "NAK",
        22 => "SYN",
        23 => "ETB",
        24 => "CAN",
        25 => "EM",
        26 => "SUB",
        27 => "ESC",
        28 => "FS",
        29 => "GS",
        30 => "RS",
        31 => "US",
        32 => "SPACE",
        127 => "DEL",
        128..=159 => return format!("C1-{code}"),
        160 => "NBSP",
        _ => return char::from_u32(u32::from(code)).unwrap().to_string(),
    };

    label.to_string()
}

fn render_table(rows: &[AsciiRow]) -> String {
    let mut table = String::from("Dec Hex  Character\n");

    for row in rows {
        table.push_str(&format!(
            "{:>3} {:<4} {}\n",
            row.decimal, row.hexadecimal, row.character
        ));
    }

    table
}

#[cfg(test)]
mod tests {
    use super::{build_rows, character_label, parse_range, render_table};

    #[test]
    fn labels_control_and_printable_characters() {
        assert_eq!(character_label(0), "NUL");
        assert_eq!(character_label(9), "TAB");
        assert_eq!(character_label(32), "SPACE");
        assert_eq!(character_label(65), "A");
        assert_eq!(character_label(127), "DEL");
    }

    #[test]
    fn accepts_decimal_and_hexadecimal_ranges() {
        assert_eq!(
            parse_range("32-34").unwrap().collect::<Vec<_>>(),
            [32, 33, 34]
        );
        assert_eq!(
            parse_range("0x41-0x42").unwrap().collect::<Vec<_>>(),
            [65, 66]
        );
        assert!(parse_range("5-2").is_err());
    }

    #[test]
    fn renders_a_stable_table_layout() {
        let rows = build_rows(0..=2);
        assert_eq!(
            render_table(&rows),
            "Dec Hex  Character\n  0 0x00 NUL\n  1 0x01 SOH\n  2 0x02 STX\n"
        );
    }

    #[test]
    fn supports_extended_latin1_rows() {
        let rows = build_rows(160..=160);
        assert_eq!(rows[0].character, "NBSP");
    }
}
