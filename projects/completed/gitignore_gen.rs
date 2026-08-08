// Solution flow:
// 1. Parse comma-separated preset names, optional custom patterns, and an optional output file.
// 2. Look up each preset's patterns from static data rather than mixing data with CLI handling.
// 3. Keep the first occurrence of every pattern so output is useful and consistently ordered.
// 4. Print the generated `.gitignore` or deliberately overwrite the requested output path.
use std::{collections::HashSet, env, fs};

fn main() {
    match run(&env::args().skip(1).collect::<Vec<_>>()) {
        Ok(()) => {}
        Err(error) => eprintln!("Error: {error}"),
    }
}

fn run(arguments: &[String]) -> Result<(), String> {
    let options = parse_options(arguments)?;
    let output = compose_ignore(&options.presets, &options.custom_patterns)?;

    match options.output_path {
        Some(path) => {
            fs::write(&path, output).map_err(|error| format!("failed to write '{path}': {error}"))
        }
        None => {
            print!("{output}");
            Ok(())
        }
    }
}

struct Options {
    presets: Vec<String>,
    custom_patterns: Vec<String>,
    output_path: Option<String>,
}

fn parse_options(arguments: &[String]) -> Result<Options, String> {
    let mut presets = Vec::new();
    let mut custom_patterns = Vec::new();
    let mut output_path = None;
    let mut index = 0;

    while index < arguments.len() {
        match arguments[index].as_str() {
            "--output" => {
                index += 1;
                output_path = Some(
                    arguments
                        .get(index)
                        .ok_or_else(|| "--output needs a path".to_string())?
                        .clone(),
                );
            }
            "--stdout" => output_path = None,
            "--custom" => {
                index += 1;
                let patterns = arguments
                    .get(index)
                    .ok_or_else(|| "--custom needs comma-separated patterns".to_string())?;
                custom_patterns.extend(split_values(patterns));
            }
            value if value.starts_with("--") => return Err(format!("unknown option '{value}'")),
            value => presets.extend(split_values(value)),
        }
        index += 1;
    }

    if presets.is_empty() && custom_patterns.is_empty() {
        return Err(
            "usage: gitignore_gen <preset[,preset]> [--custom PATTERN[,PATTERN]] [--output PATH]"
                .to_string(),
        );
    }

    Ok(Options {
        presets,
        custom_patterns,
        output_path,
    })
}

fn split_values(value: &str) -> Vec<String> {
    value
        .split(',')
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(str::to_string)
        .collect()
}

const PRESETS: &[(&str, &[&str])] = &[
    ("rust", &["/target/", "Cargo.lock", "**/*.rs.bk"]),
    ("node", &["node_modules/", "npm-debug.log*", ".env"]),
    ("python", &["__pycache__/", "*.py[cod]", ".venv/", ".env"]),
    ("go", &["/bin/", "*.test", "coverage.out"]),
    ("java", &["*.class", "/target/", ".idea/"]),
];

fn compose_ignore(presets: &[String], custom_patterns: &[String]) -> Result<String, String> {
    let mut ordered_patterns = Vec::new();
    let mut seen = HashSet::new();

    for preset in presets {
        let normalized = preset.to_ascii_lowercase();
        let (_, patterns) = PRESETS
            .iter()
            .find(|(name, _)| *name == normalized)
            .ok_or_else(|| {
                format!("unknown preset '{preset}'; available: rust, node, python, go, java")
            })?;
        add_unique_patterns(&mut ordered_patterns, &mut seen, patterns.iter().copied());
    }

    add_unique_patterns(
        &mut ordered_patterns,
        &mut seen,
        custom_patterns.iter().map(String::as_str),
    );

    Ok(ordered_patterns.join("\n") + "\n")
}

fn add_unique_patterns<'a>(
    output: &mut Vec<&'a str>,
    seen: &mut HashSet<&'a str>,
    patterns: impl IntoIterator<Item = &'a str>,
) {
    for pattern in patterns {
        if seen.insert(pattern) {
            output.push(pattern);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::compose_ignore;

    #[test]
    fn combines_presets_in_the_requested_order() {
        let presets = ["rust".to_string(), "node".to_string()];
        assert_eq!(
            compose_ignore(&presets, &[]),
            Ok(
                "/target/\nCargo.lock\n**/*.rs.bk\nnode_modules/\nnpm-debug.log*\n.env\n"
                    .to_string()
            )
        );
    }

    #[test]
    fn removes_duplicates_while_preserving_first_occurrence() {
        let presets = ["node".to_string(), "python".to_string()];
        let custom = [".env".to_string(), "*.local".to_string()];
        assert_eq!(
            compose_ignore(&presets, &custom),
            Ok(
                "node_modules/\nnpm-debug.log*\n.env\n__pycache__/\n*.py[cod]\n.venv/\n*.local\n"
                    .to_string()
            )
        );
    }

    #[test]
    fn reports_unknown_presets() {
        assert!(compose_ignore(&["unknown".to_string()], &[]).is_err());
    }
}
