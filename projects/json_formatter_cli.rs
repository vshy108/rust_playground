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

// Extra:

// - [x] pretty print (serde_json::to_string_pretty)
// - [ ] validate-only mode (parse without re-serialising)

use serde_json::{from_str, to_string_pretty};
use std::fs::read_to_string;

// Step
// 0. parse CLI args
// std::env::args() returns an iterator of the process arguments; collect() turns it into Vec<String>.
// args[0] is the program name, so iter().skip(1) starts at the first user-supplied argument.
// Returns Ok(path) if a path argument is present, Err("missing path") otherwise.
fn parse_args(args: &[String]) -> Result<String, String> {
    let mut iter = args.iter().skip(1);
    match iter.next() {
        Some(arg) => return Ok(arg.to_string()),
        None => return Err("missing path".to_string()),
    }
}

// 1. read from file path
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
    let arg = parse_args(&args).unwrap_or_else(|err| {
        eprintln!("parse_args error: {err}");
        std::process::exit(1);
    });
    let raw_string = read_file_from_path(arg.as_str())?;
    let json_string = parse_json(raw_string.as_str())?;
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

        assert_eq!(parse_args(&args), Ok("/tmp/test.json".to_string()));
    }

    #[test]
    fn return_missing_path_if_no_argument() {
        let args = vec![PROGRAM.to_string()];

        assert_eq!(parse_args(&args), Err("missing path".to_string()));
    }

     #[test]
    fn pretty_printed_output_contains_newlines_and_indentation() {
        let json_string = parse_json(r#"{"a":1}"#).unwrap();
        let pretty_json_string = format_json(&json_string).unwrap();
        // empty object is valid json, but it will fail this
        assert!(pretty_json_string.contains("\n  "));
    }
}
