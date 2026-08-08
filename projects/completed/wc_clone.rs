// Solution flow:
// 1. Parse optional `wc`-style flags and an optional file path from the command line.
// 2. Read bytes from that file or standard input, preserving the exact byte count.
// 3. Derive line, word, byte, and optional UTF-8 character counts in a pure helper.
// 4. Select the requested columns, format them like `wc`, and print the source label.
use std::{
    env, fs,
    io::{self, Read},
};

fn main() {
    match run(&env::args().skip(1).collect::<Vec<_>>()) {
        Ok(output) => print!("{output}"),
        Err(error) => eprintln!("Error: {error}"),
    }
}

fn run(arguments: &[String]) -> Result<String, String> {
    let options = parse_options(arguments)?;
    let bytes = read_input(options.path.as_deref())?;
    let counts = count_bytes(&bytes)?;
    Ok(format_counts(
        &counts,
        options.fields,
        options.path.as_deref().unwrap_or("-"),
    ))
}

#[derive(Clone, Copy)]
struct Fields {
    lines: bool,
    words: bool,
    bytes: bool,
    characters: bool,
}

impl Fields {
    fn default_counts() -> Self {
        Self {
            lines: true,
            words: true,
            bytes: true,
            characters: false,
        }
    }

    fn any(self) -> bool {
        self.lines || self.words || self.bytes || self.characters
    }
}

struct Options {
    fields: Fields,
    path: Option<String>,
}

fn parse_options(arguments: &[String]) -> Result<Options, String> {
    let mut fields = Fields {
        lines: false,
        words: false,
        bytes: false,
        characters: false,
    };
    let mut path = None;

    for argument in arguments {
        match argument.as_str() {
            "-l" | "--lines" => fields.lines = true,
            "-w" | "--words" => fields.words = true,
            "-c" | "--bytes" => fields.bytes = true,
            "-m" | "--chars" => fields.characters = true,
            value if value.starts_with('-') && value.len() > 2 => {
                for flag in value[1..].chars() {
                    match flag {
                        'l' => fields.lines = true,
                        'w' => fields.words = true,
                        'c' => fields.bytes = true,
                        'm' => fields.characters = true,
                        _ => return Err(format!("unknown flag '-{flag}'")),
                    }
                }
            }
            value => {
                if path.replace(value.to_string()).is_some() {
                    return Err("only one input file is supported".to_string());
                }
            }
        }
    }

    if !fields.any() {
        fields = Fields::default_counts();
    }
    Ok(Options { fields, path })
}

fn read_input(path: Option<&str>) -> Result<Vec<u8>, String> {
    match path {
        Some(path) => fs::read(path).map_err(|error| format!("failed to read '{path}': {error}")),
        None => {
            let mut bytes = Vec::new();
            io::stdin()
                .read_to_end(&mut bytes)
                .map_err(|error| format!("failed to read standard input: {error}"))?;
            Ok(bytes)
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Counts {
    lines: usize,
    words: usize,
    bytes: usize,
    characters: usize,
}

fn count_bytes(bytes: &[u8]) -> Result<Counts, String> {
    let text =
        std::str::from_utf8(bytes).map_err(|_| "input is not valid UTF-8 text".to_string())?;
    Ok(Counts {
        lines: bytes.iter().filter(|&&byte| byte == b'\n').count(),
        words: text.split_whitespace().count(),
        bytes: bytes.len(),
        characters: text.chars().count(),
    })
}

fn format_counts(counts: &Counts, fields: Fields, label: &str) -> String {
    let mut values = Vec::new();
    if fields.lines {
        values.push(counts.lines.to_string());
    }
    if fields.words {
        values.push(counts.words.to_string());
    }
    if fields.bytes {
        values.push(counts.bytes.to_string());
    }
    if fields.characters {
        values.push(counts.characters.to_string());
    }
    values.push(label.to_string());
    values.join(" ") + "\n"
}

#[cfg(test)]
mod tests {
    use super::{count_bytes, format_counts, parse_options, Counts, Fields};

    #[test]
    fn counts_lines_words_and_bytes_from_a_fixture() {
        assert_eq!(
            count_bytes(b"hello world\nsecond line\n"),
            Ok(Counts {
                lines: 2,
                words: 4,
                bytes: 24,
                characters: 24,
            })
        );
    }

    #[test]
    fn counts_utf8_characters_separately_from_bytes() {
        let counts = count_bytes("é\n".as_bytes()).unwrap();
        assert_eq!(counts.bytes, 3);
        assert_eq!(counts.characters, 2);
    }

    #[test]
    fn supports_combined_output_flags() {
        let options = parse_options(&["-lw".to_string(), "notes.txt".to_string()]).unwrap();
        assert!(options.fields.lines);
        assert!(options.fields.words);
        assert!(!options.fields.bytes);
        assert_eq!(options.path.as_deref(), Some("notes.txt"));
    }

    #[test]
    fn formats_selected_values_and_label() {
        let counts = Counts {
            lines: 2,
            words: 4,
            bytes: 24,
            characters: 24,
        };
        assert_eq!(
            format_counts(
                &counts,
                Fields {
                    lines: true,
                    words: true,
                    bytes: true,
                    characters: false,
                },
                "sample.txt"
            ),
            "2 4 24 sample.txt\n"
        );
    }
}
