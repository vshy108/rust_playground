use std::{env, fs, path::Path};

fn main() {
    match run(&env::args().skip(1).collect::<Vec<_>>()) {
        Ok(output) => print!("{output}"),
        Err(error) => eprintln!("Error: {error}"),
    }
}

enum SplitMode {
    Lines(usize),
    Bytes(usize),
}

fn run(arguments: &[String]) -> Result<String, String> {
    if arguments.first().map(String::as_str) == Some("--join") {
        let manifest = arguments
            .get(1)
            .ok_or_else(|| "usage: file_splitter --join MANIFEST OUTPUT".to_string())?;
        let output = arguments
            .get(2)
            .ok_or_else(|| "usage: file_splitter --join MANIFEST OUTPUT".to_string())?;
        if arguments.len() != 3 {
            return Err("usage: file_splitter --join MANIFEST OUTPUT".to_string());
        }
        return join_manifest(manifest, output);
    }
    let (mode, path, write_manifest) = parse_options(arguments)?;
    let input = fs::read(path).map_err(|error| format!("failed to read '{path}': {error}"))?;
    let parts = match mode {
        SplitMode::Lines(limit) => split_lines(&input, limit),
        SplitMode::Bytes(limit) => split_bytes(&input, limit),
    };
    let mut output = String::new();
    for (index, part) in parts.iter().enumerate() {
        let destination = part_name(path, index + 1);
        fs::write(&destination, part)
            .map_err(|error| format!("failed to write '{destination}': {error}"))?;
        output.push_str(&format!("{destination}\n"));
    }
    if write_manifest {
        let manifest = format!("{path}.manifest");
        fs::write(&manifest, &output)
            .map_err(|error| format!("failed to write '{manifest}': {error}"))?;
        output.push_str(&format!("{manifest}\n"));
    }
    Ok(output)
}

fn parse_options(arguments: &[String]) -> Result<(SplitMode, &str, bool), String> {
    let usage = "usage: file_splitter (--lines N | --bytes N) FILE [--manifest]";
    let (flag, value, path, write_manifest) = match arguments {
        [flag, value, path] => (flag, value, path, false),
        [flag, value, path, manifest] if manifest == "--manifest" => (flag, value, path, true),
        _ => return Err(usage.to_string()),
    };
    let limit = value
        .parse::<usize>()
        .map_err(|_| "split size must be a positive number".to_string())?;
    if limit == 0 {
        return Err("split size must be greater than zero".to_string());
    }
    let mode = match flag.as_str() {
        "--lines" => SplitMode::Lines(limit),
        "--bytes" => SplitMode::Bytes(limit),
        _ => return Err(usage.to_string()),
    };
    Ok((mode, path, write_manifest))
}

fn join_manifest(manifest: &str, output: &str) -> Result<String, String> {
    let entries = fs::read_to_string(manifest)
        .map_err(|error| format!("failed to read manifest '{manifest}': {error}"))?;
    let mut content = Vec::new();
    for part in entries.lines().filter(|line| !line.is_empty()) {
        content.extend(
            fs::read(part).map_err(|error| format!("failed to read part '{part}': {error}"))?,
        );
    }
    fs::write(output, content).map_err(|error| format!("failed to write '{output}': {error}"))?;
    Ok(format!("reassembled {output}\n"))
}

fn split_bytes(input: &[u8], limit: usize) -> Vec<Vec<u8>> {
    input.chunks(limit).map(ToOwned::to_owned).collect()
}

fn split_lines(input: &[u8], limit: usize) -> Vec<Vec<u8>> {
    let mut parts = Vec::new();
    let mut current = Vec::new();
    let mut lines = 0;
    for line in input.split_inclusive(|byte| *byte == b'\n') {
        current.extend_from_slice(line);
        lines += 1;
        if lines == limit {
            parts.push(std::mem::take(&mut current));
            lines = 0;
        }
    }
    if !current.is_empty() {
        parts.push(current);
    }
    parts
}

fn part_name(path: &str, index: usize) -> String {
    let path = Path::new(path);
    let stem = path
        .file_stem()
        .and_then(|value| value.to_str())
        .unwrap_or("part");
    let extension = path.extension().and_then(|value| value.to_str());
    match extension {
        Some(extension) => format!("{stem}.part{index:03}.{extension}"),
        None => format!("{stem}.part{index:03}"),
    }
}

#[cfg(test)]
mod tests {
    use super::{part_name, split_bytes, split_lines};

    #[test]
    fn splits_bytes_at_exact_boundaries() {
        assert_eq!(
            split_bytes(b"abcdef", 3),
            vec![b"abc".to_vec(), b"def".to_vec()]
        );
        assert_eq!(
            split_bytes(b"abcde", 3),
            vec![b"abc".to_vec(), b"de".to_vec()]
        );
    }

    #[test]
    fn splits_lines_without_merging_boundaries() {
        assert_eq!(
            split_lines(b"one\ntwo\nthree\n", 2),
            vec![b"one\ntwo\n".to_vec(), b"three\n".to_vec()]
        );
    }

    #[test]
    fn names_parts_deterministically() {
        assert_eq!(part_name("notes.txt", 2), "notes.part002.txt");
        assert_eq!(part_name("archive", 12), "archive.part012");
    }
}
