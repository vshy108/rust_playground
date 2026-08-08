use std::{env, fs};

const FENCE: &str = "\x60\x60\x60";

fn main() {
    match run(&env::args().skip(1).collect::<Vec<_>>()) {
        Ok(output) => print!("{output}"),
        Err(error) => eprintln!("Error: {error}"),
    }
}

#[derive(Debug, Default)]
struct Options {
    width: usize,
    section: Option<String>,
    toc: bool,
}

fn run(arguments: &[String]) -> Result<String, String> {
    let path = arguments.first().ok_or_else(|| {
        "usage: markdown_reader FILE [--width N] [--section TITLE] [--toc]".to_string()
    })?;
    let options = parse_options(arguments)?;
    let input =
        fs::read_to_string(path).map_err(|error| format!("failed to read '{path}': {error}"))?;
    if options.toc {
        return Ok(table_of_contents(&input));
    }
    let selected = if let Some(section) = options.section {
        section_body(&input, &section)?
    } else {
        input
    };
    Ok(render(&selected, options.width))
}

fn parse_options(arguments: &[String]) -> Result<Options, String> {
    let mut options = Options {
        width: 80,
        ..Options::default()
    };
    let mut index = 1;
    while index < arguments.len() {
        match arguments[index].as_str() {
            "--toc" => options.toc = true,
            "--width" | "--section" => {
                let value = arguments
                    .get(index + 1)
                    .ok_or_else(|| format!("{} needs a value", arguments[index]))?;
                if arguments[index] == "--width" {
                    options.width = value
                        .parse()
                        .map_err(|_| "width must be a positive integer".to_string())?;
                    if options.width == 0 {
                        return Err("width must be a positive integer".to_string());
                    }
                } else {
                    options.section = Some(value.clone());
                }
                index += 1;
            }
            value => return Err(format!("unknown option '{value}'")),
        }
        index += 1;
    }
    Ok(options)
}

fn render(input: &str, width: usize) -> String {
    let mut output = String::new();
    let mut in_code = false;
    for raw in input.lines() {
        let line = raw.trim_end();
        if line.trim_start().starts_with(FENCE) {
            in_code = !in_code;
            continue;
        }
        if in_code {
            output.push_str("    ");
            output.push_str(line.trim());
            output.push('\n');
        } else if let Some(title) = line.strip_prefix("# ") {
            output.push_str(&title.to_uppercase());
            output.push('\n');
            output.push_str(&"=".repeat(title.chars().count().min(width)));
            output.push('\n');
        } else if let Some(title) = line.strip_prefix("## ") {
            output.push_str(title);
            output.push('\n');
            output.push_str(&"-".repeat(title.chars().count().min(width)));
            output.push('\n');
        } else if let Some(item) = line.strip_prefix("- ").or_else(|| line.strip_prefix("* ")) {
            output.push_str("• ");
            output.push_str(item);
            output.push('\n');
        } else if line.is_empty() {
            output.push('\n');
        } else {
            for wrapped in wrap(line, width) {
                output.push_str(&wrapped);
                output.push('\n');
            }
        }
    }
    output
}

fn wrap(line: &str, width: usize) -> Vec<String> {
    let mut lines = Vec::new();
    let mut current = String::new();
    for word in line.split_whitespace() {
        if !current.is_empty() && current.chars().count() + 1 + word.chars().count() > width {
            lines.push(current);
            current = String::new();
        }
        if !current.is_empty() {
            current.push(' ');
        }
        current.push_str(word);
    }
    if !current.is_empty() {
        lines.push(current);
    }
    lines
}

fn section_body(input: &str, wanted: &str) -> Result<String, String> {
    let mut found = false;
    let mut output = String::new();
    for line in input.lines() {
        if let Some(title) = line.strip_prefix("## ") {
            if found {
                break;
            }
            found = title.trim() == wanted;
        }
        if found {
            output.push_str(line);
            output.push('\n');
        }
    }
    if found {
        Ok(output)
    } else {
        Err(format!("section '{wanted}' not found"))
    }
}

fn table_of_contents(input: &str) -> String {
    input
        .lines()
        .filter_map(|line| line.strip_prefix("# ").or_else(|| line.strip_prefix("## ")))
        .map(|title| format!("- {title}\n"))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::{FENCE, render, section_body, table_of_contents, wrap};

    #[test]
    fn renders_headings_lists_and_code() {
        let input = format!("# Title\n- item\n{FENCE}text\nlet x = 1;\n{FENCE}\n");
        let output = render(&input, 80);
        assert_eq!(output, "TITLE\n=====\n• item\n    let x = 1;\n");
    }

    #[test]
    fn wraps_without_splitting_words() {
        assert_eq!(wrap("one two three", 7), vec!["one two", "three"]);
    }

    #[test]
    fn supports_section_jumps_and_toc() {
        let input = "# Intro\ntext\n## Install\nstep\n## Usage\nrun\n";
        assert_eq!(section_body(input, "Usage").unwrap(), "## Usage\nrun\n");
        assert_eq!(table_of_contents(input), "- Intro\n- Install\n- Usage\n");
    }
}
