// example must have main function
// Goal: Ownership + iterators

// Build:

// ```bash
// rgrep "hello" logs.txt
// ```

// Learn:

// - borrowing
//   - References let you use a value without taking ownership.
//   - &str is a borrowed string slice; String is an owned heap string.
//   - Functions that only read data should take &str, not String.
//
// - iterators
//   - Iterators are lazy: they produce values on demand, not all at once.
//   - .lines() splits a &str into an iterator of &str lines.
//   - .filter() keeps only items where the closure returns true.
//   - .enumerate() pairs each item with its 0-based index.
//   - .collect() pulls all iterator values into a Vec or other collection.
//
// - slices
//   - A slice (&[T] or &str) is a view into a contiguous sequence.
//   - args: &[String] is a slice of the collected CLI arg Vec.
//   - Slices do not own their data; they borrow it.

// Progress:

// 1. Result<Config, String>: parse_args returns Ok(Config) or Err(message).
//    `if let Ok(...) = ...` cannot bind Err — use `match` to handle both arms.
// 2. Named lifetimes: `fn foo<'a>(x: &'a str) -> impl Iterator<Item = &'a str>`
//    tells Rust the output borrows from `x`, not from some other input.
//    '_ is a shorthand when there is only one input borrow to infer from.
// 3. move closures: `move |x| ...` copies captured variables into the closure.
//    Needed when the closure outlives the function (e.g. returned iterators).
//    &str is Copy, so `move` copies the fat pointer, not the string data.
// 4. .enumerate() order matters: enumerate before filter preserves original line
//    numbers. enumerate after filter resets the index to match count only.
//    Each item becomes (usize, T); use _ to ignore unused parts of the tuple.
// 5. Testing iterators: iterators can't be compared directly — .collect::<Vec<_>>()
//    materialises them into a Vec so assert_eq! can compare element by element.
//    Test through public functions (filter_lines_with_pattern), not main.

// Extra:

// - [ ] regex support (use the `regex` crate)

#[derive(Debug, PartialEq)]
struct Config {
    pattern: String,
    file_path: String,
}

fn parse_args(args: &[String]) -> Result<Config, String> {
    if let Some(pattern) = args.get(1)
        && let Some(file_path) = args.get(2)
    {
        Ok(Config {
            pattern: pattern.to_string(),
            file_path: file_path.to_string(),
        })
    } else {
        Err("Usage: rgrep <pattern> <file>".to_string())
    }
}

fn read_file_content(file_path: &String) -> Result<String, std::io::Error> {
    std::fs::read_to_string(file_path)
}

// core::str::Lines<'_>: the iterator type returned by .lines().
// '_ is a lifetime placeholder — it tells Rust this iterator borrows from `contents`.
// Equivalent to writing <'a>(contents: &'a str) -> core::str::Lines<'a>,
// but Rust infers the lifetime so we write '_ as shorthand.
fn split_contents_to_lines(contents: &str) -> core::str::Lines<'_> {
    contents.lines()
}

// .enumerate() before .filter() preserves original file line numbers.
// After .enumerate(), each item is (usize, &str) — filtering on the tuple means
// the index is still the original line position, not the match count.
// `_` ignores the index inside the filter closure (we only need it for printing).
// Return type is (usize, &'a str): caller uses index + 1 for 1-based line numbers.
// 'a ties the output lifetime to `lines`; `move` copies the &str `pattern` into
// the closure so it doesn't borrow from this function's stack frame.
fn filter_lines_with_pattern<'a>(
    lines: core::str::Lines<'a>,
    pattern: &str,
) -> impl Iterator<Item = (usize, &'a str)> {
    lines
        .enumerate()
        .filter(move |(_, line)| line.contains(pattern))
}

// Iterates over (0-based index, line) tuples from filter_lines_with_pattern.
// Prints each as "N: line content" where N is 1-based (index + 1).
// Takes ownership of the iterator — consuming it drives the lazy chain.
fn print_lines_with_line_numbers<'a>(filter_lines: impl Iterator<Item = (usize, &'a str)>) {
    filter_lines.for_each(|(index, line)| println!("{}: {}", index + 1, line));
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    // if let... else cannot bind the Err value, need use match
    match parse_args(&args) {
        Ok(Config { pattern, file_path }) => {
            let contents = read_file_content(&file_path).unwrap_or_else(|e| {
                eprintln!("Error reading file: {}", e);
                std::process::exit(1);
            });
            let lines = split_contents_to_lines(&contents);
            let filter_lines = filter_lines_with_pattern(lines, &pattern);
            print_lines_with_line_numbers(filter_lines)
        }
        Err(msg) => {
            eprintln!("{}", msg);
            std::process::exit(1);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const PROGRAM: &str = "rgrep";

    #[test]
    fn return_lines_match_pattern() {
        // .collect::<Vec<_>>() is needed because iterators can't be compared directly.
        // The index is 0-based here — 1-based formatting happens only in print_lines_with_line_numbers.
        // "goodbye" is at index 1 and does not contain "hello", so it is absent from results.
        let lines = split_contents_to_lines("hello world\ngoodbye\nhello rust");
        let result = filter_lines_with_pattern(lines, "hello").collect::<Vec<_>>();
        assert_eq!(result.len(), 2);
        assert_eq!(result[0].0, 0);
        assert_eq!(result[0].1, "hello world");
        assert_eq!(result[1].0, 2);
        assert_eq!(result[1].1, "hello rust");
    }

    #[test]
    fn return_empty_results_when_no_match() {
        let lines = split_contents_to_lines("hello world\ngoodbye\nhello rust");
        let result = filter_lines_with_pattern(lines, "hellowa").collect::<Vec<_>>();
        assert!(result.is_empty());
    }

    #[test]
    fn missing_args_returns_usage_error() {
        // args[0] is always the program name; pattern and file are args[1] and args[2].
        // Passing only the program name should return Err with the usage message.
        let args = vec![PROGRAM.to_string()];
        assert!(parse_args(&args).is_err());
    }
}
