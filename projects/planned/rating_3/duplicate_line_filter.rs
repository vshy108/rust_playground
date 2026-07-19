// Solution flow:
// 1. Parse normalization, keep-last, count, and statistics flags plus an optional input file.
// 2. Read the input as lines and build a comparison key for each line.
// 3. Keep either the first or last occurrence for every key while retaining input order.
// 4. Render the selected lines, optional occurrence counts, and optional stream statistics.
use std::{
    collections::{HashMap, HashSet},
    env, fs,
    io::{self, Read},
};

fn main() {
    match run(&env::args().skip(1).collect::<Vec<_>>()) {
        Ok((output, statistics)) => {
            print!("{output}");
            if let Some(statistics) = statistics {
                eprintln!("{statistics}");
            }
        }
        Err(error) => eprintln!("Error: {error}"),
    }
}

fn run(arguments: &[String]) -> Result<(String, Option<String>), String> {
    let options = parse_options(arguments)?;
    let input = read_input(options.path.as_deref())?;
    let result = filter_lines(&input, options);
    let output = render_lines(&result.lines, &result.counts, options.show_counts);
    let statistics = options.show_statistics.then(|| {
        format!(
            "lines: {}, unique: {}, duplicates removed: {}",
            result.total_lines,
            result.lines.len(),
            result.total_lines - result.lines.len()
        )
    });
    Ok((output, statistics))
}

#[derive(Clone, Copy)]
struct Options<'a> {
    ignore_case: bool,
    trim_whitespace: bool,
    keep_last: bool,
    show_counts: bool,
    show_statistics: bool,
    path: Option<&'a str>,
}

fn parse_options(arguments: &[String]) -> Result<Options<'_>, String> {
    let mut options = Options {
        ignore_case: false,
        trim_whitespace: false,
        keep_last: false,
        show_counts: false,
        show_statistics: false,
        path: None,
    };

    for argument in arguments {
        match argument.as_str() {
            "--ignore-case" => options.ignore_case = true,
            "--trim" => options.trim_whitespace = true,
            "--keep-last" => options.keep_last = true,
            "--counts" => options.show_counts = true,
            "--stats" => options.show_statistics = true,
            value if value.starts_with('-') => return Err(format!("unknown option '{value}'")),
            path => {
                if options.path.replace(path).is_some() {
                    return Err("only one input file is supported".to_string());
                }
            }
        }
    }

    Ok(options)
}

fn read_input(path: Option<&str>) -> Result<String, String> {
    match path {
        Some(path) => {
            fs::read_to_string(path).map_err(|error| format!("failed to read '{path}': {error}"))
        }
        None => {
            let mut input = String::new();
            io::stdin()
                .read_to_string(&mut input)
                .map_err(|error| format!("failed to read standard input: {error}"))?;
            Ok(input)
        }
    }
}

struct FilterResult {
    lines: Vec<String>,
    counts: Vec<usize>,
    total_lines: usize,
}

fn filter_lines(input: &str, options: Options<'_>) -> FilterResult {
    let source_lines = input.lines().map(str::to_string).collect::<Vec<_>>();
    let mut occurrence_counts = HashMap::new();
    for line in &source_lines {
        *occurrence_counts
            .entry(comparison_key(line, options))
            .or_insert(0) += 1;
    }

    let indexes = if options.keep_last {
        keep_last_indexes(&source_lines, options)
    } else {
        keep_first_indexes(&source_lines, options)
    };
    let lines = indexes
        .iter()
        .map(|&index| source_lines[index].clone())
        .collect::<Vec<_>>();
    let counts = indexes
        .iter()
        .map(|&index| occurrence_counts[&comparison_key(&source_lines[index], options)])
        .collect();

    FilterResult {
        lines,
        counts,
        total_lines: source_lines.len(),
    }
}

fn keep_first_indexes(lines: &[String], options: Options<'_>) -> Vec<usize> {
    let mut seen = HashSet::new();
    lines
        .iter()
        .enumerate()
        .filter_map(|(index, line)| seen.insert(comparison_key(line, options)).then_some(index))
        .collect()
}

fn keep_last_indexes(lines: &[String], options: Options<'_>) -> Vec<usize> {
    let mut seen = HashSet::new();
    let mut indexes = lines
        .iter()
        .enumerate()
        .rev()
        .filter_map(|(index, line)| seen.insert(comparison_key(line, options)).then_some(index))
        .collect::<Vec<_>>();
    indexes.reverse();
    indexes
}

fn comparison_key(line: &str, options: Options<'_>) -> String {
    let normalized = if options.trim_whitespace {
        line.trim()
    } else {
        line
    };
    if options.ignore_case {
        normalized.to_lowercase()
    } else {
        normalized.to_string()
    }
}

fn render_lines(lines: &[String], counts: &[usize], show_counts: bool) -> String {
    lines
        .iter()
        .zip(counts)
        .map(|(line, count)| {
            if show_counts {
                format!("{count}\t{line}\n")
            } else {
                format!("{line}\n")
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::{filter_lines, render_lines, Options};

    fn options() -> Options<'static> {
        Options {
            ignore_case: false,
            trim_whitespace: false,
            keep_last: false,
            show_counts: false,
            show_statistics: false,
            path: None,
        }
    }

    #[test]
    fn keeps_first_duplicate_by_default() {
        let result = filter_lines("red\nblue\nred\ngreen\nblue\n", options());
        assert_eq!(result.lines, ["red", "blue", "green"]);
        assert_eq!(result.counts, [2, 2, 1]);
    }

    #[test]
    fn supports_case_and_whitespace_normalization() {
        let mut options = options();
        options.ignore_case = true;
        options.trim_whitespace = true;
        let result = filter_lines(" Rust\nrust\nRUST \n", options);
        assert_eq!(result.lines, [" Rust"]);
    }

    #[test]
    fn keeps_last_occurrence_in_original_order() {
        let mut options = options();
        options.keep_last = true;
        let result = filter_lines("a\nb\na\nc\nb\n", options);
        assert_eq!(result.lines, ["a", "c", "b"]);
    }

    #[test]
    fn renders_occurrence_counts() {
        assert_eq!(render_lines(&["red".to_string()], &[3], true), "3\tred\n");
    }
}
