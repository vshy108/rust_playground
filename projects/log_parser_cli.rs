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
// - `iter()` vs `into_iter()` vs `iter_mut()` — three ways to iterate a collection:
//   - `.iter()`      → yields `&T`      (shared borrow);  collection stays alive; use for read-only access
//   - `.iter_mut()`  → yields `&mut T`  (mutable borrow); collection stays alive; use to modify in place
//   - `.into_iter()` → yields `T`       (owned value);    collection is consumed;  use when done with it
//   - rule: if you need to return data from a function, it must be owned — use `.into_iter()`, not `.iter()`
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

// - [x] CSV export — ip_stats (count + mean per IP) + write_csv (header + rows)

use std::collections::HashMap;
use std::io::Write;
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

// FIX 1: `&Vec<LogEntry>` → `&[LogEntry]`: Clippy prefers slices over &Vec; slices are more general
//         (any contiguous sequence works, not just Vec) and avoid a redundant indirection.
// FIX 2: return type was `std::iter::Take<std::slice::Iter<'_, (&IpAddr, &usize)>>` — a borrow of a
//         local variable. Local variables are dropped when the function returns, so the borrow would
//         dangle. An owned `Vec<(IpAddr, usize)>` has no lifetime dependency and can be returned safely.
fn top_ips(entries: &[LogEntry]) -> Vec<(IpAddr, usize)> {
    // Step 1: count requests per IP using .entry().or_insert(0) to avoid a separate if/else.
    let mut counts: HashMap<IpAddr, usize> = HashMap::new();
    for entry in entries {
        *counts.entry(entry.ip).or_insert(0) += 1;
    }
    // Step 2: convert to Vec for sorting.
    // ❌ counts.iter()      → (&IpAddr, &usize): borrows from `counts`; can't return — `counts` is local
    // ✅ counts.into_iter() → (IpAddr, usize):   owned values; safe to return after `counts` is consumed
    // FIX 3: was `counts.iter().collect::<Vec<(&IpAddr, &usize)>>()` — iter() yields references that
    //         borrow from `counts`; those references can't outlive the local `counts`. into_iter()
    //         consumes `counts` and yields owned (IpAddr, usize) tuples with no lifetime attachment.
    let mut vec: Vec<(IpAddr, usize)> = counts.into_iter().collect();
    // Step 3: sort descending by count. b.1.cmp(&a.1) reverses order vs a.1.cmp(&b.1).
    vec.sort_by(|a, b| b.1.cmp(&a.1)); // b before a = descending
    // Step 4: take top 5 and collect into owned Vec so there are no dangling borrows.
    // FIX 4: was `vec.into_iter().take(5)` — a lazy iterator, not a Vec; the return type promised Vec.
    //         .collect() drives the iterator and materialises the result into the declared return type.
    vec.into_iter().take(5).collect()
}

// Returns None if entries is empty (no meaningful stats to compute).
// Returns Some((mean_ms, p99_ms)) otherwise.
//   mean — arithmetic average; sensitive to outliers (one 10s request pulls it up)
//   p99  — value at the 99th percentile; the slowest 1% of requests are above this
fn latency_stats(entries: &[LogEntry]) -> Option<(f64, u64)> {
    // Early return keeps the happy path flat — no else block needed after a return.
    if entries.is_empty() {
        return None;
    }

    // Collect into Vec<u64> so we can sort in place for the p99 calculation.
    // .map(|e| e.latency_ms) extracts the field; .collect() drives the lazy iterator.
    let mut latencies: Vec<u64> = entries.iter().map(|e| e.latency_ms).collect();
    // .sort() works without a comparator because u64 implements Ord (total ordering).
    // sort_by is only needed when the comparison is non-standard (e.g. descending, floats).
    latencies.sort();

    let n = latencies.len();
    // turbofish ::<u64> tells the compiler which numeric type to accumulate into;
    // without it, .sum() is ambiguous — u32, u64, i64 all implement Sum.
    let sum = latencies.iter().sum::<u64>();
    // cast to f64 before dividing so integer truncation doesn't lose the fractional part;
    // e.g. 21 / 2 = 10 in u64, but 21.0 / 2.0 = 10.5 in f64.
    let mean = sum as f64 / n as f64;

    // p99 index: for n=20 → 20*99/100 = 19 (last element ≈ max for small samples).
    // integer division is intentional: n*99/100 gives the floor index into the sorted vec.
    let p99_index = n * 99 / 100;
    let p99 = latencies[p99_index];

    Some((mean, p99))
}

// Returns None if entries is empty — dividing by zero total would give NaN, not a meaningful rate.
// Returns Some(percentage) otherwise, e.g. 25.0 means 25% of requests were 5xx errors.
fn error_rate(entries: &[LogEntry]) -> Option<f64> {
    if entries.is_empty() {
        return None;
    }
    let total = entries.len();
    // .filter() keeps only entries matching the predicate; .count() drives the iterator and tallies.
    // status_code >= 500 && < 600 covers all 5xx codes (500, 502, 503, 504, …).
    // status_code < 600 is technically redundant for well-formed logs but makes the intent explicit.
    let status_5xx_count = entries
        .iter()
        .filter(|e| e.status_code >= 500 && e.status_code < 600)
        .count();

    // Cast both counts to f64 before dividing — integer division would truncate to 0 for any
    // error rate below 100%. Multiply by 100.0 to express as a percentage.
    Some(status_5xx_count as f64 / total as f64 * 100.0)
}

// Aggregates per-IP statistics in a single pass over entries.
// Returns one tuple per unique IP: (ip, request_count, mean_latency_ms).
//
// Value type is (usize, u64) — count and latency sum — so a single .entry() call
// accumulates both fields without a second HashMap or a second loop.
fn ip_stats(entries: &[LogEntry]) -> Vec<(IpAddr, usize, f64)> {
    let mut map: HashMap<IpAddr, (usize, u64)> = HashMap::new();
    for e in entries {
        // or_insert((0, 0)) returns &mut (usize, u64) for the existing or freshly inserted slot.
        let slot = map.entry(e.ip).or_insert((0, 0));
        slot.0 += 1; // count
        slot.1 += e.latency_ms; // latency sum
    }
    // Consume map with into_iter() — iter() would yield borrowed (&IpAddr, &(usize, u64))
    // that can't outlive the local `map`. into_iter() yields owned values, safe to return.
    // Divide inside the map closure so callers receive the final mean directly.
    map.into_iter()
        .map(|(ip, (count, sum))| (ip, count, sum as f64 / count as f64))
        .collect()
}

// Writes a CSV file at `path` with one header row and one data row per IP.
// File::create truncates any existing file at that path before writing.
// writeln! returns io::Result; the ? operator propagates any write error to the caller.
// {mean:.2} rounds the float to two decimal places in the output (e.g. 45.67).
fn write_csv(path: &str, stats: &[(IpAddr, usize, f64)]) -> Result<(), std::io::Error> {
    let mut file = std::fs::File::create(path)?;
    writeln!(file, "ip,requests,mean_latency_ms")?;
    for (ip, count, mean) in stats {
        writeln!(file, "{ip},{count},{mean:.2}")?;
    }
    Ok(())
}

fn main() {
    // skip(1) drops argv[0] (the binary name) so all indices below are 0-based over user args.
    let args: Vec<String> = std::env::args().skip(1).collect();

    // windows(2) slides a two-element view [flag, value] across the args vec.
    // find returns the first window where the flag matches; map extracts the value after it.
    // Result: Some("out.csv") if --csv was passed, None otherwise.
    let csv_path = args
        .windows(2)
        .find(|w| w[0] == "--csv")
        .map(|w| w[1].clone());

    // Treat the first arg that doesn't start with "--" as the log file path.
    // This lets flags appear in any order without breaking positional arg logic.
    let path_arg = args
        .iter()
        .find(|a| !a.starts_with("--"))
        .cloned()
        .unwrap_or("./fixtures/access.log".to_string());
    let contents = read_file_content(&path_arg).unwrap_or_else(|e| {
        eprintln!("Error reading file: {}", e);
        std::process::exit(1);
    });
    let lines = split_contents_to_lines(&contents);
    // .collect() drives the lazy iterator — without it, parse_line never runs.
    let entries: Vec<LogEntry> = parse_entries(lines).collect();

    if let Some(ref path) = csv_path {
        let stats = ip_stats(&entries);
        write_csv(path, &stats).unwrap_or_else(|e| eprintln!("csv error: {e}"));
        println!("wrote csv: {path}");
    }
    println!("parsed {} log entries", entries.len());
    println!("top 5: {:?}", top_ips(&entries));
    // latency_stats returns Option — if entries is empty there is nothing to print.
    // if let unpacks Some((mean, p99)) in one step and skips the None case silently.
    if let Some((mean, p99)) = latency_stats(&entries) {
        println!("mean latency: {:.1}ms  p99: {}ms", mean, p99);
    }

    if let Some(error_rate) = error_rate(&entries) {
        // error_rate returns Option — None if entries is empty; if let skips that case safely.
        println!("error rate: {}%", error_rate);
    }
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

    #[test]
    fn return_top_ips_from_logs() {
        let entries: Vec<LogEntry> = vec![
            LogEntry {
                ip: "2.2.3.4".parse().unwrap(),
                method: HttpMethod::Get,
                path: String::new(),
                status_code: 200,
                latency_ms: 0,
            },
            LogEntry {
                ip: "1.2.3.4".parse().unwrap(),
                method: HttpMethod::Get,
                path: String::new(),
                status_code: 200,
                latency_ms: 0,
            },
            LogEntry {
                ip: "1.2.3.4".parse().unwrap(),
                method: HttpMethod::Get,
                path: String::new(),
                status_code: 200,
                latency_ms: 0,
            },
            LogEntry {
                ip: "1.2.3.4".parse().unwrap(),
                method: HttpMethod::Get,
                path: String::new(),
                status_code: 200,
                latency_ms: 0,
            },
        ];
        let top_entries = top_ips(&entries);
        assert_eq!(top_entries.len(), 2);
        // FIX: `.parse()` alone is ambiguous — `FromStr` is implemented by many types, so the
        // compiler can't infer which one to use. `assert_eq!` infers each side independently;
        // even though `.0` is `IpAddr`, that fact doesn't flow back into `.parse()`.
        // Turbofish `::<IpAddr>` pins the target type explicitly so the compiler can resolve it.
        assert_eq!(top_entries[0].0, "1.2.3.4".parse::<IpAddr>().unwrap());
        assert_eq!(top_entries[0].1, 3);
        assert_eq!(top_entries[1].0, "2.2.3.4".parse::<IpAddr>().unwrap());
        assert_eq!(top_entries[1].1, 1);
    }

    #[test]
    fn top_ips_returns_none_for_empty_slice() {
        assert!(top_ips(&[]).is_empty());
    }

    #[test]
    fn error_rate_counts_5xx_as_percentage() {
        // 1 error out of 4 total = 25.0% exactly — safe to assert with == on f64 because
        // 25.0 is representable exactly in binary floating point (25 = 11001 in binary).
        // Avoid fractions like 1/3 or 1/6 whose binary representations are infinite repeating.
        let entries = vec![
            LogEntry {
                ip: "1.2.3.4".parse().unwrap(),
                method: HttpMethod::Get,
                path: String::new(),
                status_code: 200, // ok
                latency_ms: 0,
            },
            LogEntry {
                ip: "1.2.3.4".parse().unwrap(),
                method: HttpMethod::Get,
                path: String::new(),
                status_code: 200, // ok
                latency_ms: 0,
            },
            LogEntry {
                ip: "1.2.3.4".parse().unwrap(),
                method: HttpMethod::Get,
                path: String::new(),
                status_code: 200, // ok
                latency_ms: 0,
            },
            LogEntry {
                ip: "1.2.3.4".parse().unwrap(),
                method: HttpMethod::Get,
                path: String::new(),
                status_code: 500, // error — this is the one 5xx
                latency_ms: 0,
            },
        ];

        // unwrap() is safe here: slice is non-empty, so error_rate always returns Some.
        let rate = error_rate(&entries).unwrap();
        assert_eq!(rate, 25.0);
    }

    #[test]
    fn error_rate_returns_none_for_empty_slice() {
        assert!(error_rate(&[]).is_none());
    }

    #[test]
    fn ip_stats_returns_count_and_mean_latency() {
        // Two requests from 1.2.3.4 with latencies 10 and 30 → mean = 20.0 (exactly representable).
        // One request from 2.2.3.4 with latency 50 → mean = 50.0.
        let entries = vec![
            LogEntry {
                ip: "1.2.3.4".parse().unwrap(),
                method: HttpMethod::Get,
                path: String::new(),
                status_code: 200,
                latency_ms: 10,
            },
            LogEntry {
                ip: "1.2.3.4".parse().unwrap(),
                method: HttpMethod::Get,
                path: String::new(),
                status_code: 200,
                latency_ms: 30,
            },
            LogEntry {
                ip: "2.2.3.4".parse().unwrap(),
                method: HttpMethod::Get,
                path: String::new(),
                status_code: 200,
                latency_ms: 50,
            },
        ];

        let mut stats = ip_stats(&entries);
        // HashMap iteration order is non-deterministic; sort by IP string for a stable assertion.
        stats.sort_by_key(|(ip, _, _)| ip.to_string());

        assert_eq!(stats.len(), 2);
        assert_eq!(stats[0].0, "1.2.3.4".parse::<IpAddr>().unwrap());
        assert_eq!(stats[0].1, 2);    // count
        assert_eq!(stats[0].2, 20.0); // mean latency
        assert_eq!(stats[1].0, "2.2.3.4".parse::<IpAddr>().unwrap());
        assert_eq!(stats[1].1, 1);
        assert_eq!(stats[1].2, 50.0);
    }

    #[test]
    fn write_csv_produces_correct_rows() {
        // Write to a temp file and read it back to verify the content.
        let path = "/tmp/logparse_test_out.csv";
        let stats: Vec<(IpAddr, usize, f64)> = vec![
            ("1.2.3.4".parse().unwrap(), 3, 25.5),
            ("2.2.3.4".parse().unwrap(), 1, 50.0),
        ];

        write_csv(path, &stats).expect("write_csv should not fail");

        let contents = std::fs::read_to_string(path).expect("should read temp file");
        let lines: Vec<&str> = contents.lines().collect();

        // Header row must be first.
        assert_eq!(lines[0], "ip,requests,mean_latency_ms");
        // Data rows — mean rounded to 2 decimal places.
        assert_eq!(lines[1], "1.2.3.4,3,25.50");
        assert_eq!(lines[2], "2.2.3.4,1,50.00");
        assert_eq!(lines.len(), 3);
    }
}
