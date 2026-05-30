// example must have main function
// Goal: Rust serialization with serde

// Build:

// ```bash
// jsonfmt input.json
// ```

// Learn:

// - `serde` / `serde_json`
// - `Result` and error propagation with `?`
// - file IO with `std::fs`
// - `Box<dyn Error>` as a catch-all error type

// Progress:

// 1. Box<dyn Error> as main's return error type
//    - Error is a trait; both std::io::Error and serde_json::Error implement it.
//    - dyn Error means "any type implementing Error, resolved at runtime" (trait object).
//    - Box is needed because dyn Error has no known size at compile time;
//      Box puts it on the heap and gives back a fixed-size pointer.
//    - Returning Result<(), Box<dyn Error>> from main allows ? to propagate
//      any error that implements Error, regardless of its concrete type.
//
// 2. ? operator (error propagation shorthand)
//    - On Ok(T): unwraps to T and continues.
//    - On Err(E): immediately returns Err(E) from the current function.
//    - Requires the function return type to be Result (or Option).
//    - Replaces verbose match { Ok(v) => v, Err(e) => return Err(e) } boilerplate.
//
// 3. raw string literals r#"..."#
//    - Lets you write " inside a string without escaping (no backslashes needed).
//    - Syntax: r#"content"# — the number of # must match on both ends.
//    - Useful for JSON test strings: r#"{"a":1}"# vs "{\"a\":1}"
//    - r means raw (escape sequences like \n are NOT processed inside).
//
// 4. if let vs match on Option
//    - iter.next() returns Option<T>: Some(T) if a value exists, None if exhausted.
//    - `if let Some(x) = expr { ... }` — use when you only care about the Some branch.
//    - `match expr { Some(x) => ..., None => ... }` — use when both branches need handling.
//    - `let Some(x) = expr { ... }` is NOT valid syntax; if let is the correct form.
//
// 5. impl Trait in function parameters
//    - `fn f(x: impl Read)` means "any concrete type that implements Read".
//    - The compiler generates a separate copy of the function for each concrete type used
//      (monomorphisation) — no runtime overhead, unlike `dyn Trait` (dynamic dispatch).
//    - `mut reader: impl Read` — `mut` applies to the local binding, not the caller's value;
//      needed here because `read_to_string` takes `&mut self`.
//    - Enables passing io::stdin() in production and Cursor<&[u8]> in tests, same function.
//    - std::io::Cursor<T>: wraps an in-memory buffer and adds a position pointer so it
//      satisfies the Read trait. Cursor::new(b"...") lets tests inject fake stdin without
//      touching the real process IO.

// Extra:

// - [x] pretty print (serde_json::to_string_pretty)
// - [x] validate-only mode (parse without re-serialising)

use serde_json::{from_str, to_string_pretty};
use std::fs::read_to_string;
// `self` brings std::io into scope as `io`, enabling `io::stdin()`.
// `Read` brings the Read trait into scope; required to call `.read_to_string()`
// on any impl Read value (the method lives on the trait, not the concrete type).
use std::io::{self, Read};

#[derive(Debug, PartialEq)]
struct Config {
    file_path: Option<String>,
    is_check: bool,
}

// Step
// 0. parse CLI args
// std::env::args() returns an iterator of the process arguments; collect() turns it into Vec<String>.
// args[0] is the program name, so iter().skip(1) starts at the first user-supplied argument.
// Returns Ok(Config) always: file_path is Some(path) if given, None if omitted (stdin mode).
// is_check is true only if "--check" flag is present.
fn parse_args(args: &[String]) -> Result<Config, String> {
    let mut iter = args.iter().skip(1);
    let mut file_path = "";
    let mut is_check = false;
    while let Some(arg) = iter.next() {
        // FIX: arg is &String; matching on arg.as_str() gives &str so string
        // literal patterns like "--check" (also &str) can match.
        match arg.as_str() {
            "--check" => is_check = true,
            // `other` captures the &str value produced by arg.as_str() —
            // same content as arg but already the right type for file_path.
            other => {
                file_path = other;
            }
        }
    }

    // If file_path was set during the loop, return a Config; otherwise the
    // user gave no path argument.
    if file_path != "" {
        Ok(Config {
            file_path: Some(file_path.to_string()),
            is_check,
        })
    } else {
        Ok(Config {
            file_path: None,
            is_check,
        })
    }
}

// 1a. read from any reader (file, stdin, or in-memory buffer in tests)
// Accepts any type implementing Read (io::stdin(), fs::File, Cursor<&[u8]>, etc.).
// Reads all bytes into a String and returns it.
fn read_input(mut reader: impl Read) -> Result<String, std::io::Error> {
    let mut buf = String::new();
    reader.read_to_string(&mut buf)?;
    Ok(buf)
}

// 1b. read from file path
// fs::read_to_string opens the file, reads all bytes, and returns them as a String.
// On any IO error (file not found, permission denied, etc.) it returns Err(io::Error).
fn read_file_from_path(path: &str) -> Result<String, std::io::Error> {
    read_to_string(path)
}

// 2. parse content as json
// serde_json::from_str deserialises raw text into a serde_json::Value.
// Value is an enum that can represent any valid JSON (Object, Array, String, Number, Bool, Null).
// Returns Err(serde_json::Error) if the text is not valid JSON.
fn parse_json(content: &str) -> Result<serde_json::Value, serde_json::Error> {
    from_str(content)
}

// 3. format json
// serde_json::to_string_pretty serialises the Value back to a String with 2-space indentation.
// Keys in objects are sorted alphabetically (BTreeMap ordering).
// Returns Err(serde_json::Error) only on unencodable values (very rare for Value).
fn format_json(value: &serde_json::Value) -> Result<String, serde_json::Error> {
    to_string_pretty(value)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    let Config {
        file_path,
        is_check,
    } = parse_args(&args).unwrap_or_else(|err| {
        eprintln!("parse_args error: {err}");
        std::process::exit(1);
    });
    // FIX: raw_string must be declared outside the match so it is accessible below.
    // Each arm evaluates to the String; `let raw_string = match` binds it in the outer scope.
    let raw_string = match file_path {
        Some(path) => read_file_from_path(&path)?,
        // read_input accepts any impl Read; io::stdin() satisfies that in production.
        None => read_input(io::stdin())?,
    };

    // match instead of ? so --check mode can exit with the right code before pretty-printing.
    let json_string = match parse_json(raw_string.as_str()) {
        Ok(value) => {
            if is_check {
                // Valid JSON in check mode: exit 0 (success) without printing.
                std::process::exit(0)
            } else {
                // FIX: `return value` would exit main, not bind json_string.
                // A match arm in `let x = match` must evaluate to the value to bind.
                // `return` exits the whole function; bare `value` is the arm's result.
                value
            }
        }
        Err(error) => {
            if is_check {
                // Invalid JSON in check mode: exit 1 (failure) without printing.
                std::process::exit(1);
            }
            // FIX: `throw` does not exist in Rust.
            // `return Err(e.into())` propagates the error out of main.
            // `.into()` converts serde_json::Error into Box<dyn Error>,
            // which is what main's return type requires.
            return Err(error.into());
        }
    };

    let pretty_json_string = format_json(&json_string)?;
    println!("{}", pretty_json_string);

    return Ok(());
}

#[cfg(test)]
mod tests {
    use super::*;

    const PROGRAM: &str = "jsonfmt";

    #[test]
    fn parsing_valid_json_object_returns_ok() {
        let result = parse_json(r#"{"a":1}"#);
        assert!(result.is_ok());
    }

    #[test]
    fn parsing_inalid_json_object_returns_error() {
        let result = parse_json(r#"{a":1}"#);
        assert!(result.is_err());
    }

    #[test]
    fn extract_string_from_second_argument() {
        let args = vec![PROGRAM.to_string(), "/tmp/test.json".to_string()];

        assert_eq!(
            parse_args(&args),
            Ok(Config {
                file_path: Some("/tmp/test.json".to_string()),
                is_check: false,
            })
        );
    }

    #[test]
    fn no_path_sets_file_path_to_none() {
        let args = vec![PROGRAM.to_string()];

        assert_eq!(
            parse_args(&args),
            Ok(Config {
                file_path: None,
                is_check: false,
            })
        );
    }

    #[test]
    fn check_flag_sets_is_check_true() {
        let args = vec![
            PROGRAM.to_string(),
            "--check".to_string(),
            "/tmp/test.json".to_string(),
        ];

        assert_eq!(
            parse_args(&args),
            Ok(Config {
                file_path: Some("/tmp/test.json".to_string()),
                is_check: true,
            })
        );
    }

    #[test]
    fn no_check_flag_sets_is_check_false() {
        let args = vec![PROGRAM.to_string(), "/tmp/test.json".to_string()];

        assert_eq!(
            parse_args(&args),
            Ok(Config {
                file_path: Some("/tmp/test.json".to_string()),
                is_check: false,
            })
        );
    }

    #[test]
    fn read_input_reads_all_bytes_from_reader() {
        // Cursor<&[u8]> implements Read, so it can stand in for stdin in tests.
        let cursor = std::io::Cursor::new(r#"{"a":1}"#);
        let result = read_input(cursor);
        assert_eq!(result.unwrap(), r#"{"a":1}"#);
    }

    #[test]
    fn pretty_printed_output_contains_newlines_and_indentation() {
        let json_string = parse_json(r#"{"a":1}"#).unwrap();
        let pretty_json_string = format_json(&json_string).unwrap();
        // empty object is valid json, but it will fail this
        assert!(pretty_json_string.contains("\n  "));
    }
}
