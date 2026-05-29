// example must have main function
// Goal: Rust basics

// Build:

// ```bash
// genpass --length 20
// ```

// Learn:

// - Cargo
// - `String`
// - `Vec`
// - `rand`
// - argument parsing

// Extra:

// - support symbols toggle

use rand::Rng;

// Ok with usize, Err with String, not &str because it from runtime input
fn parse_args(args: &[String]) -> Result<(usize, bool), String> {
    // read args and find the length
    // get the 2nd element and wrap out from Some
    if let Some(first_option) = args.get(1) {
        if first_option == "--length" {
            if let Some(second_option) = args.get(2) {
                // try parse to ~i32~ usize because fuction argument type is usize
                match second_option.parse::<usize>() {
                    Ok(length) => {
                        if length >= 1 {
                            Ok((length, false))
                        } else {
                            Err("not a valid usize".to_string())
                        }
                    }
                    // cannot Err(error: ParseIntError) but let error: ParseIntError = error;
                    Err(error) => {
                        // invalid digit found in string
                        Err(format!("not a valid usize: {error}"))
                    }
                }
            } else {
                Err("missing flag value".to_string())
            }
        } else {
            Err(format!("invalid flag: {first_option}"))
        }
    } else {
        Ok((10, false))
    }
}

fn build_charset() -> Vec<char> {
    // Printable ASCII characters are from byte 33 to 126.
    // !"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\]^_`abcdefghijklmnopqrstuvwxyz{|}~
    // letters
    // numbers
    // punctuation
    // symbols
    (33u8..=126u8).map(char::from).collect()
}

// function order no matter before or after main, but before is better
fn generate_password(length: usize) -> String {
    let chars = build_charset();
    // random_range needs to mutate rng internally so the next random
    // number is different from the previous one.
    // current random state -> generate number -> update state -> next state
    let mut rng = rand::rng();
    // know the final size, can pre-allocate
    let mut password = String::with_capacity(length);
    // Vec::len() is very cheap.
    // It just reads a stored number from the Vec; it does not count the elements one by one
    // avoid even that tiny repeated call
    let chars_len = chars.len();

    for _ in 0..length {
        let index = rng.random_range(0..chars_len);
        password.push(chars[index]);
    }

    password
}

// cargo run --bin genpass -- --length 20
// genpass --length 20
fn main() {
    let args: Vec<String> = std::env::args().collect();
    // std::env::args() Before collect()
    // Args { inner: ["target/debug/genpass", "--length", "20"] }
    // Args { inner: ["genpass", "--length", "20"] }
    // after collect then Vec but the Vec need explicitly mention type
    // pass by reference
    let (length, has_symbol) = parse_args(&args).unwrap_or_else(|err| {
        eprintln!("parse_args error: {err}");
        std::process::exit(1);
    });
    println!("{:?}, {}", generate_password(length), has_symbol);
}

#[cfg(test)]
mod tests {
    // super::generate_password
    use super::*;

    #[test]
    fn generates_password_with_requested_length() {
        let password = generate_password(20);

        assert_eq!(password.len(), 20);
    }

    #[test]
    fn generated_password_uses_printable_ascii_without_space() {
        let password = generate_password(100);

        // check all fulfill condition
        // char can use range also
        assert!(password.chars().all(|ch| ('!'..='~').contains(&ch)));
    }

    #[test]
    fn parses_length_argument() {
        let args = vec![
            "genpass".to_string(),
            "--length".to_string(),
            "20".to_string(),
        ];

        assert_eq!(parse_args(&args), Ok((20, false)));
    }

    #[test]
    fn returns_missing_flag_value_when_length_value_is_missing() {
        let args = vec!["genpass".to_string(), "--length".to_string()];

        assert_eq!(parse_args(&args), Err("missing flag value".to_string()));
    }

    #[test]
    fn returns_invalid_usize_when_length_value_is_invalid() {
        let args = vec![
            "genpass".to_string(),
            "--length".to_string(),
            "ko".to_string(),
        ];

        assert_eq!(
            parse_args(&args),
            Err("not a valid usize: invalid digit found in string".to_string())
        );
    }

    #[test]
    fn returns_invalid_usize_when_length_value_is_zero() {
        let args = vec![
            "genpass".to_string(),
            "--length".to_string(),
            "0".to_string(),
        ];

        assert_eq!(parse_args(&args), Err("not a valid usize".to_string()));
    }

    #[test]
    fn returns_invalid_usize_when_length_value_is_float_number() {
        let args = vec![
            "genpass".to_string(),
            "--length".to_string(),
            "1.2".to_string(),
        ];

        assert_eq!(
            parse_args(&args),
            Err("not a valid usize: invalid digit found in string".to_string())
        );
    }

    #[test]
    fn returns_invalid_usize_when_length_value_is_negative_integer() {
        let args = vec![
            "genpass".to_string(),
            "--length".to_string(),
            "-1".to_string(),
        ];

        assert_eq!(
            parse_args(&args),
            Err("not a valid usize: invalid digit found in string".to_string())
        );
    }

    #[test]
    fn returns_default_value_ten_when_no_flag() {
        let args = vec!["genpass".to_string()];

        assert_eq!(parse_args(&args), Ok((10, false)));
    }

    #[test]
    fn returns_invalid_flag_with_flag_when_flag_is_invalid() {
        let args = vec!["genpass".to_string(), "--ko".to_string(), "1".to_string()];

        assert_eq!(parse_args(&args), Err("invalid flag: --ko".to_string()));
    }

    #[test]
    fn builds_printable_ascii_charset_without_space() {
        let chars = build_charset();

        assert_eq!(chars.first(), Some(&'!'));
        assert_eq!(chars.last(), Some(&'~'));
        assert_eq!(chars.len(), 94);
        assert!(!chars.contains(&' '));
    }
}
