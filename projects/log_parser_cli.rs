// example must have main function
// Goal: Real backend work

// Build:

// ```bash
// cargo run --bin logparse -- fixtures/access.log
// ```

// Learn:

// - iterators — chaining map/filter/fold to aggregate log data without intermediate allocations
// - parsing — splitting lines into fields; using split_once and FromStr for typed values
// - aggregation — HashMap counters for per-IP counts; running totals for latency and errors
// - `HashMap` vs `BTreeMap` vs `Vec + sort_by` — choosing the right collection:
//   - `HashMap<K, V>`   — fast O(1) insert/lookup; no iteration order; use for counting and grouping
//   - `BTreeMap<K, V>`  — O(log n) insert/lookup; always iterates in ascending key order; use when
//                         you need ordered-by-key traversal (e.g. sorted output by IP string)
//   - `Vec + sort_by`   — sort by any field, including value (count); use for top-N-by-value queries
//   - pattern for top-N: `HashMap` to count → `Vec` to sort by value → `.take(N)` for top entries
// - `.entry(key).or_insert(0)` — HashMap method that returns &mut V for an existing key, or inserts
//   a default and returns &mut V for a new key; dereference and increment: `*map.entry(k).or_insert(0) += 1`
// - `&str` vs `String` — both represent text, but ownership differs:
//   - `&str`   — borrowed view into existing memory; no allocation; cannot outlive its source
//   - `String` — heap-allocated, owned copy; can be stored in structs and returned from functions
//   - convert `&str` → `String` with `.to_string()` or `.to_owned()`; this copies the bytes onto the heap
//   - use `&str` for read-only access; use `String` when the value must be owned (e.g. struct fields)
//   - temporary lifetime pitfall: `read_to_string(...).lines()` does not compile — `.lines()` borrows
//     from the `String`, but the `String` is a temporary with no binding and is dropped immediately;
//     fix: bind to `let contents` first, then call `contents.lines()` so the `String` outlives the iterator
// - `regex::Regex` vs `regex::bytes::Regex`
//   - `regex::Regex`       — input is `&str` (UTF-8 text); captures are `&str`; use for log files
//   - `regex::bytes::Regex` — input is `&[u8]` (raw bytes); captures are `&[u8]`; use for binary/network data
// - `std::net::IpAddr` — enum covering both address families:
//   - `IpAddr::V4(Ipv4Addr)` — four u8 octets; "127.0.0.1".parse::<IpAddr>() → Ok(V4(...))
//   - `IpAddr::V6(Ipv6Addr)` — sixteen u8 octets; "::1".parse::<IpAddr>() → Ok(V6(...))
//   - implements `FromStr`, so any valid IPv4 or IPv6 string parses directly; invalid input returns Err
// - `std::sync::LazyLock<T>` — initializes a value on first access, then caches it for all future accesses
//   - useful for `static` variables whose value can only be computed at runtime (e.g. compiled `Regex`)
//   - `LazyLock::new(|| { ... })` takes a closure that runs exactly once; result is stored and reused
//   - thread-safe: if two threads race on first access, only one runs the closure; the other waits
// - raw strings `r"..."` / `r#"..."#` — string literals where backslashes are not escape sequences
//   - `r"..."` — basic raw string; cannot contain a literal `"` inside
//   - `r#"..."#` — raw string delimited by `#`; can contain `"` freely; use when the string itself has quotes
//   - `r##"..."##` — add more `#` pairs if the content contains `"#`; the rule is: delimiters must not
//     appear inside the string, so add as many `#` as needed to make the delimiters unique

// Progress:

// 1. `FromStr` trait — implement `from_str(s: &str) -> Result<Self, Self::Err>` on a type
//    to unlock `.parse::<T>()` on any `&str`; `type Err` is the associated error type returned on failure

// Extra:

// - [ ] CSV export

use std::net::IpAddr;
use std::str::FromStr;
use std::sync::LazyLock;

use regex::Regex;

// `FromStr` is a trait from std::str that lets a type be constructed from a string slice.
// Implementing it unlocks the `.parse::<T>()` method on any `&str`.
//
// Required items:
//   type Err  — the error type returned when parsing fails
//   fn from_str(s: &str) -> Result<Self, Self::Err>  — the parsing logic
//
// Usage:
//   "GET".parse::<HttpMethod>()   →  Ok(HttpMethod::Get)
//   "BREW".parse::<HttpMethod>()  →  Err("unknown HTTP method: BREW")
//
// The compiler also uses FromStr when the target type can be inferred:
//   let m: HttpMethod = "GET".parse().unwrap();
#[derive(Debug, PartialEq)]
enum HttpMethod {
    Get,
    Post,
    Put,
    Patch,
    Delete,
    Head,
    Options,
}

impl FromStr for HttpMethod {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "GET" => Ok(HttpMethod::Get),
            "POST" => Ok(HttpMethod::Post),
            "PUT" => Ok(HttpMethod::Put),
            "PATCH" => Ok(HttpMethod::Patch),
            "DELETE" => Ok(HttpMethod::Delete),
            "HEAD" => Ok(HttpMethod::Head),
            "OPTIONS" => Ok(HttpMethod::Options),
            other => Err(format!("unknown HTTP method: {other}")),
        }
    }
}

#[derive(Debug)]
struct LogEntry {
    ip: IpAddr,
    method: HttpMethod,
    path: String,
    status_code: u16,
    latency_ms: u64,
}

// Compiled once on first use and reused for every line — avoids re-compiling the pattern
// on each `parse_line` call.
//
// Pattern breakdown (Common Log Format):
//   ^(\S+)          — group 1: IP address (no spaces)
//   " - - "          — literal auth fields we don't need
//   \[.*?\]          — timestamp in brackets, matched non-greedily (.*? stops at first ])
//   " "(\S+)         — group 2: HTTP method (GET, POST, …)
//   " "(\S+)         — group 3: request path (/api/users)
//   " \S+"           — HTTP version (HTTP/1.1), matched but not captured
//   " "(\d+)         — group 4: status code (digits only)
//   " \d+"           — response bytes, matched but not captured
//   " "(\d+)$        — group 5: latency in ms (digits only, end of line)
static RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r#"^(\S+) - - \[.*?\] "(\S+) (\S+) \S+" (\d+) \d+ (\d+)$"#).unwrap()
});

fn read_file_content(file_path: &str) -> Result<String, std::io::Error> {
    std::fs::read_to_string(file_path)
}

fn split_contents_to_lines(contents: &str) -> core::str::Lines<'_> {
    contents.lines()
}

// Common Log Format:
//   IP - - [timestamp] "METHOD path HTTP/1.1" status bytes latency_ms
//
// Returns None for any line that doesn't match the expected shape,
// so the caller can skip malformed lines without stopping.
fn parse_line(line: &str) -> Option<LogEntry> {
    // RE.captures returns None if the line doesn't match the pattern;
    // ? propagates None immediately so the rest of the body is skipped.
    let caps = RE.captures(line)?;

    // caps[n] indexes into the capture groups (1-based; 0 is the whole match).
    // Each caps[n] is &str — .parse() uses FromStr, .ok() converts Err to None,
    // and ? short-circuits if the value is invalid (bad IP, unknown method, etc.).
    let ip: IpAddr = caps[1].parse().ok()?;
    let method: HttpMethod = caps[2].parse().ok()?;
    // String's FromStr never fails, so .to_string() is more idiomatic than .parse().ok()?.
    let path = caps[3].to_string();
    let status_code: u16 = caps[4].parse().ok()?;
    let latency_ms: u64 = caps[5].parse().ok()?;

    // Field shorthand: variable names match struct field names, so no `ip: ip` needed.
    Some(LogEntry {
        ip,
        method,
        path,
        status_code,
        latency_ms,
    })
}

// .filter_map(parse_line) applies parse_line to each line and keeps only the Some(LogEntry)
// values — None (malformed lines) are silently skipped. This combines .map + .filter in one step.
fn parse_entries(lines: core::str::Lines<'_>) -> impl Iterator<Item = LogEntry> {
    lines.filter_map(parse_line)
}

fn main() {
    let path_arg = std::env::args()
        .nth(1)
        .unwrap_or("./fixtures/access.log".to_string());
    let contents = read_file_content(&path_arg).unwrap_or_else(|e| {
        eprintln!("Error reading file: {}", e);
        std::process::exit(1);
    });
    let lines = split_contents_to_lines(&contents);
    // .collect() drives the lazy iterator — without it, parse_line never runs.
    let entries: Vec<LogEntry> = parse_entries(lines).collect();
    println!("parsed {} log entries", entries.len());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_valid_get_line() {
        let line =
            r#"127.0.0.1 - - [02/Jun/2026:10:00:00 +0000] "GET /api/users HTTP/1.1" 200 512 23"#;
        let entry = parse_line(line).expect("should parse");

        assert_eq!(entry.ip, "127.0.0.1".parse::<IpAddr>().unwrap());
        assert_eq!(entry.method, HttpMethod::Get);
        assert_eq!(entry.path, "/api/users");
        assert_eq!(entry.status_code, 200);
        assert_eq!(entry.latency_ms, 23);
    }

    #[test]
    fn parses_valid_post_line() {
        let line =
            r#"10.0.0.5 - - [02/Jun/2026:10:00:01 +0000] "POST /api/orders HTTP/1.1" 201 128 45"#;
        let entry = parse_line(line).expect("should parse");

        assert_eq!(entry.method, HttpMethod::Post);
        assert_eq!(entry.status_code, 201);
        assert_eq!(entry.latency_ms, 45);
    }

    #[test]
    fn returns_none_for_malformed_line() {
        assert!(parse_line("this line is malformed and should be skipped").is_none());
    }

    #[test]
    fn returns_none_for_unknown_method() {
        let line = r#"127.0.0.1 - - [02/Jun/2026:10:00:00 +0000] "BREW /coffee HTTP/1.1" 418 0 1"#;
        assert!(parse_line(line).is_none());
    }

    #[test]
    fn parse_entries_skips_malformed_lines() {
        // Three lines: two valid, one malformed. parse_entries must return exactly 2 entries.
        let input = [
            r#"127.0.0.1 - - [02/Jun/2026:10:00:00 +0000] "GET /a HTTP/1.1" 200 0 10"#,
            "this is not a log line",
            r#"10.0.0.1 - - [02/Jun/2026:10:00:01 +0000] "POST /b HTTP/1.1" 201 0 20"#,
        ]
        .join("\n");

        let entries: Vec<LogEntry> = parse_entries(input.lines()).collect();

        assert_eq!(entries.len(), 2);
        assert_eq!(entries[0].path, "/a");
        assert_eq!(entries[1].path, "/b");
    }
}
