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

// Progress:

// 1. Cargo — build, run, test with `cargo`
// 2. `String` vs `&str` — use `String` for runtime-constructed values (e.g. error messages)
// 3. `Vec` — collect iterators; pre-allocate with `with_capacity`
// 4. `rand` — `rand::rng()` must be `mut`; state advances on each call
// 5. Argument parsing — `std::env::args()` returns an iterator; `.collect()` needs explicit `Vec<String>` type
// 6. `Result<T, E>` — use `Ok(val)` / `Err(msg)`; `unwrap_or_else` handles errors without panicking
// 7. Pattern matching type annotation — `Err(e: ParseIntError)` is invalid in match arms;
//    use `let e: ParseIntError = e;` inside the arm body to assert the type instead
// 8. Tuple in `Result` — `Result<(usize, bool), String>` packs multiple success values;
//    destructure with `let (length, symbols) = ...`
// 9. `Vec::len()` is O(1) — reads a stored count, does not iterate
// 10. `usize` parse — rejects floats (`3.14`), negatives (`-1`), letters; all give "invalid digit"
// 11. `if` is an expression — both branches must return the same type; no `else` means the implicit type is `()`
// 12. `.chain()` — joins two iterators into one sequence; ranges cannot be combined with `+`
// 13. `build_charset(with_symbols: bool)` — alphanumeric (62 chars) vs full printable ASCII (94 chars)
// 14. `while let Some(x) = iter.next()` — loop until iterator exhausted; same as `if let` but repeats
// 15. Manual iterator advance — calling `iter.next()` inside the loop body consumes the next token,
//     enabling flag+value pairs (`--length 20`) without positional indexing

// Extra:

// - [x] support symbols toggle

use rand::Rng;

// Returns Ok((length, symbols)) or Err(message).
// Err variant holds a String (not &str) because some messages are built at runtime with format!.
fn parse_args(args: &[String]) -> Result<(usize, bool), String> {
    // rewrite with a mut iterator loop to support --symbols before or after --length.
    // skip(1) to drop the program name
    let mut arg_iter = args.iter().skip(1);
    let mut length = 10;
    let mut has_symbols = false;

    while let Some(arg) = arg_iter.next() {
        // &String to &str
        match arg.as_str() {
            "--length" => match arg_iter.next() {
                None => return Err("missing flag value".to_string()),
                // parse::<usize>() rejects floats, negatives, and letters — all return "invalid digit"
                Some(val) => match val.parse::<usize>() {
                    // Note: Err(error: ParseIntError) is not valid match syntax.
                    // Type annotation in a match arm must use: let e: ParseIntError = error;
                    Err(e) => return Err(format!("not a valid usize: {e}")),
                    Ok(0) => return Err("not a valid usize".to_string()),
                    Ok(n) => length = n,
                },
            },
            "--symbols" => {
                has_symbols = true;
            }
            other => return Err(format!("invalid flag: {other}")),
        }
    }

    // Result need Ok or Err
    Ok((length, has_symbols))
}

fn build_charset(with_symbols: bool) -> Vec<char> {
    // Printable ASCII characters are from byte 33 to 126.
    // !"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\]^_`abcdefghijklmnopqrstuvwxyz{|}~
    // letters 65u8..=90u8, 97u8..=122u8
    // numbers 48u8..=57u8
    // punctuation
    // symbols
    // with_symbols=true:  full printable ASCII bytes 33–126 (94 chars)
    // with_symbols=false: alphanumeric only — 0-9 (48–57) + A-Z (65–90) + a-z (97–122) = 62 chars
    // if is an expression: both branches must return the same type,
    // so else is required whenever the true branch returns a non-() value.
    if with_symbols {
        (33u8..=126u8).map(char::from).collect()
    } else {
        // Ranges can't be combined with + — use .chain() to concatenate iterators.
        (48u8..=57u8)
            .chain(65u8..=90u8)
            .chain(97u8..=122u8)
            .map(char::from)
            .collect()
    }
}

// Function definitions can appear before or after main — Rust resolves names across the whole file.
fn generate_password(length: usize, with_symbols: bool) -> String {
    let chars = build_charset(with_symbols);
    // rng must be mut: each call to random_range advances internal state to produce a new number.
    let mut rng = rand::rng();
    // with_capacity pre-allocates the exact bytes needed — no reallocations during the loop.
    let mut password = String::with_capacity(length);
    // Cache len() outside the loop — it's O(1) but there's no reason to call it on every iteration.
    let chars_len = chars.len();

    for _ in 0..length {
        let index = rng.random_range(0..chars_len);
        password.push(chars[index]);
    }

    password
}

// Run: cargo run --bin genpass -- --length 20
fn main() {
    // std::env::args() is lazy — .collect() materialises it into Vec<String>.
    // Type must be explicit: the compiler can't infer Vec<String> from collect() alone.
    // args[0] is the binary name; actual flags start at index 1.
    let args: Vec<String> = std::env::args().collect();
    // Pass by reference so parse_args borrows the slice without taking ownership.
    let (length, with_symbols) = parse_args(&args).unwrap_or_else(|err| {
        eprintln!("parse_args error: {err}");
        std::process::exit(1);
    });
    println!("{:?}", generate_password(length, with_symbols));
}

#[cfg(test)]
mod tests {
    // super::generate_password
    use super::*;

    // --- generate_password ---

    #[test]
    fn generates_password_with_requested_length() {
        let password = generate_password(20, true);

        assert_eq!(password.len(), 20);
    }

    #[test]
    fn generated_password_with_symbols_uses_full_printable_ascii() {
        let password = generate_password(200, true);

        assert!(password.chars().all(|ch| ('!'..='~').contains(&ch)));
    }

    #[test]
    fn generated_password_without_symbols_uses_only_alphanumeric() {
        let password = generate_password(200, false);

        assert!(
            password
                .chars()
                // is_ascii_alphanumeric nice
                .all(|ch| ch.is_ascii_alphanumeric())
        );
    }

    // --- build_charset ---

    #[test]
    fn builds_full_printable_ascii_charset_with_symbols() {
        let chars = build_charset(true);

        assert_eq!(chars.first(), Some(&'!'));
        assert_eq!(chars.last(), Some(&'~'));
        assert_eq!(chars.len(), 94);
        assert!(!chars.contains(&' '));
    }

    #[test]
    fn builds_alphanumeric_charset_without_symbols() {
        let chars = build_charset(false);

        assert_eq!(chars.len(), 62); // 10 digits + 26 uppercase + 26 lowercase
        assert!(!chars.contains(&'!'));
        assert!(!chars.contains(&'~'));
        assert!(chars.contains(&'0'));
        assert!(chars.contains(&'9'));
        assert!(chars.contains(&'A'));
        assert!(chars.contains(&'Z'));
        assert!(chars.contains(&'a'));
        assert!(chars.contains(&'z'));
    }

    // --- parse_args ---

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
    fn parses_symbols_flag_alone() {
        let args = vec!["genpass".to_string(), "--symbols".to_string()];

        assert_eq!(parse_args(&args), Ok((10, true)));
    }

    #[test]
    fn parses_length_then_symbols() {
        let args = vec![
            "genpass".to_string(),
            "--length".to_string(),
            "20".to_string(),
            "--symbols".to_string(),
        ];

        assert_eq!(parse_args(&args), Ok((20, true)));
    }

    #[test]
    fn parses_symbols_then_length() {
        let args = vec![
            "genpass".to_string(),
            "--symbols".to_string(),
            "--length".to_string(),
            "20".to_string(),
        ];

        assert_eq!(parse_args(&args), Ok((20, true)));
    }

    #[test]
    fn returns_default_length_with_symbols_flag() {
        let args = vec!["genpass".to_string(), "--symbols".to_string()];

        assert_eq!(parse_args(&args), Ok((10, true)));
    }

    #[test]
    fn returns_default_value_ten_when_no_flag() {
        let args = vec!["genpass".to_string()];

        assert_eq!(parse_args(&args), Ok((10, false)));
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
    fn returns_invalid_flag_with_flag_when_flag_is_invalid() {
        let args = vec!["genpass".to_string(), "--ko".to_string(), "1".to_string()];

        assert_eq!(parse_args(&args), Err("invalid flag: --ko".to_string()));
    }
}
