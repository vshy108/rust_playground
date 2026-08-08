// Solution flow:
// 1. Parse an `lf` or `crlf` target plus stdout, in-place, or directory conversion mode.
// 2. Detect the input's newline style before changing it.
// 3. Normalize all newline variants, then rebuild the text with the requested ending.
// 4. Write only files that differ from the target and report converted paths in stable order.
use std::{
    env, fs,
    path::{Path, PathBuf},
};

fn main() {
    match run(&env::args().skip(1).collect::<Vec<_>>()) {
        Ok(output) => print!("{output}"),
        Err(error) => eprintln!("Error: {error}"),
    }
}

fn run(arguments: &[String]) -> Result<String, String> {
    let Options { target, mode } = parse_options(arguments)?;
    match mode {
        Mode::StandardInput => {
            let input = std::io::read_to_string(std::io::stdin())
                .map_err(|error| format!("failed to read standard input: {error}"))?;
            Ok(convert_line_endings(&input, target))
        }
        Mode::InPlace(path) => convert_file(&path, target).map(|changed| {
            if changed {
                format!("converted {path}\n")
            } else {
                format!("already {path}\n")
            }
        }),
        Mode::Directory { root, extension } => {
            let mut paths = Vec::new();
            collect_files(Path::new(&root), extension.as_deref(), &mut paths)?;
            paths.sort();
            let converted = paths
                .iter()
                .map(|path| {
                    let changed = convert_file(path.to_string_lossy().as_ref(), target)?;
                    Ok(changed.then(|| path.display().to_string()))
                })
                .collect::<Result<Vec<Option<String>>, String>>()?
                .into_iter()
                .flatten()
                .collect::<Vec<_>>();
            Ok(converted
                .iter()
                .map(|path| format!("converted {path}\n"))
                .collect())
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum LineEnding {
    Lf,
    Crlf,
}

enum Mode {
    StandardInput,
    InPlace(String),
    Directory {
        root: String,
        extension: Option<String>,
    },
}

struct Options {
    target: LineEnding,
    mode: Mode,
}

fn parse_options(arguments: &[String]) -> Result<Options, String> {
    let (target, remaining) = arguments.split_first().ok_or_else(|| {
        "usage: line_ending_converter <lf|crlf> [--in-place FILE | --dir DIR [--extension EXT]]"
            .to_string()
    })?;
    let target = match target.as_str() {
        "lf" => LineEnding::Lf,
        "crlf" => LineEnding::Crlf,
        _ => return Err("target must be lf or crlf".to_string()),
    };

    let mode = match remaining {
        [] => Mode::StandardInput,
        [flag, path] if flag == "--in-place" => Mode::InPlace(path.clone()),
        [flag, directory] if flag == "--dir" => Mode::Directory {
            root: directory.clone(),
            extension: None,
        },
        [flag, directory, extension_flag, extension]
            if flag == "--dir" && extension_flag == "--extension" =>
        {
            Mode::Directory {
                root: directory.clone(),
                extension: Some(extension.trim_start_matches('.').to_string()),
            }
        }
        _ => return Err("invalid conversion mode".to_string()),
    };

    Ok(Options { target, mode })
}

#[derive(Debug, PartialEq, Eq)]
enum DetectedEnding {
    None,
    Lf,
    Crlf,
    Mixed,
}

fn detect_line_endings(input: &str) -> DetectedEnding {
    let crlf_count = input.match_indices("\r\n").count();
    let remaining_lf_count = input.replace("\r\n", "").matches('\n').count();
    match (crlf_count > 0, remaining_lf_count > 0) {
        (false, false) => DetectedEnding::None,
        (false, true) => DetectedEnding::Lf,
        (true, false) => DetectedEnding::Crlf,
        (true, true) => DetectedEnding::Mixed,
    }
}

fn convert_line_endings(input: &str, target: LineEnding) -> String {
    let normalized = input.replace("\r\n", "\n").replace('\r', "\n");
    match target {
        LineEnding::Lf => normalized,
        LineEnding::Crlf => normalized.replace('\n', "\r\n"),
    }
}

fn convert_file(path: &str, target: LineEnding) -> Result<bool, String> {
    let input =
        fs::read_to_string(path).map_err(|error| format!("failed to read '{path}': {error}"))?;
    if matches!(
        (detect_line_endings(&input), target),
        (DetectedEnding::Lf, LineEnding::Lf) | (DetectedEnding::Crlf, LineEnding::Crlf)
    ) {
        return Ok(false);
    }
    let converted = convert_line_endings(&input, target);
    if input == converted {
        return Ok(false);
    }
    fs::write(path, converted).map_err(|error| format!("failed to write '{path}': {error}"))?;
    Ok(true)
}

fn collect_files(
    root: &Path,
    extension: Option<&str>,
    paths: &mut Vec<PathBuf>,
) -> Result<(), String> {
    for entry in fs::read_dir(root)
        .map_err(|error| format!("failed to read '{}': {error}", root.display()))?
    {
        let path = entry
            .map_err(|error| format!("failed to read directory entry: {error}"))?
            .path();
        if path.is_dir() {
            collect_files(&path, extension, paths)?;
        } else if extension.is_none_or(|extension| {
            path.extension().and_then(|value| value.to_str()) == Some(extension)
        }) {
            paths.push(path);
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{convert_line_endings, detect_line_endings, DetectedEnding, LineEnding};

    #[test]
    fn detects_lf_crlf_and_mixed_content() {
        assert_eq!(detect_line_endings("one\ntwo\n"), DetectedEnding::Lf);
        assert_eq!(detect_line_endings("one\r\ntwo\r\n"), DetectedEnding::Crlf);
        assert_eq!(detect_line_endings("one\r\ntwo\n"), DetectedEnding::Mixed);
    }

    #[test]
    fn converts_and_preserves_trailing_newlines() {
        assert_eq!(
            convert_line_endings("one\r\ntwo\n", LineEnding::Lf),
            "one\ntwo\n"
        );
        assert_eq!(
            convert_line_endings("one\ntwo\n", LineEnding::Crlf),
            "one\r\ntwo\r\n"
        );
    }

    #[test]
    fn leaves_matching_content_unchanged() {
        let input = "already\nlf\n";
        assert_eq!(convert_line_endings(input, LineEnding::Lf), input);
    }
}
