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
    let (mode, path, write_manifest, compress) = parse_options(arguments)?;
    let input = fs::read(path).map_err(|error| format!("failed to read '{path}': {error}"))?;
    let parts = match mode {
        SplitMode::Lines(limit) => split_lines(&input, limit),
        SplitMode::Bytes(limit) => split_bytes(&input, limit),
    };
    let mut output = String::new();
    for (index, part) in parts.iter().enumerate() {
        let base = part_name(path, index + 1);
        let destination = if compress { format!("{base}.gz") } else { base };
        let bytes = if compress {
            gzip_store(part)
        } else {
            part.clone()
        };
        fs::write(&destination, bytes)
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

fn parse_options(arguments: &[String]) -> Result<(SplitMode, &str, bool, bool), String> {
    let usage = "usage: file_splitter (--lines N | --bytes N) FILE [--manifest] [--gzip]";
    if arguments.len() < 3 {
        return Err(usage.to_string());
    }
    let flag = &arguments[0];
    let value = &arguments[1];
    let path = &arguments[2];
    let mut write_manifest = false;
    let mut compress = false;
    for option in &arguments[3..] {
        match option.as_str() {
            "--manifest" => write_manifest = true,
            "--gzip" => compress = true,
            _ => return Err(usage.to_string()),
        }
    }
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
    Ok((mode, path, write_manifest, compress))
}

fn join_manifest(manifest: &str, output: &str) -> Result<String, String> {
    let entries = fs::read_to_string(manifest)
        .map_err(|error| format!("failed to read manifest '{manifest}': {error}"))?;
    let mut content = Vec::new();
    for part in entries.lines().filter(|line| !line.is_empty()) {
        let bytes =
            fs::read(part).map_err(|error| format!("failed to read part '{part}': {error}"))?;
        content.extend(if part.ends_with(".gz") {
            gzip_unstore(&bytes)?
        } else {
            bytes
        });
    }
    fs::write(output, content).map_err(|error| format!("failed to write '{output}': {error}"))?;
    Ok(format!("reassembled {output}\n"))
}

fn gzip_store(input: &[u8]) -> Vec<u8> {
    let mut output = vec![0x1f, 0x8b, 8, 0, 0, 0, 0, 0, 0, 3];
    for (index, chunk) in input.chunks(65_535).enumerate() {
        output.push(if index + 1 == input.len().div_ceil(65_535) {
            1
        } else {
            0
        });
        let length = chunk.len() as u16;
        output.extend_from_slice(&length.to_le_bytes());
        output.extend_from_slice(&(!length).to_le_bytes());
        output.extend_from_slice(chunk);
    }
    output.extend_from_slice(&crc32(input).to_le_bytes());
    output.extend_from_slice(&(input.len() as u32).to_le_bytes());
    output
}

fn gzip_unstore(input: &[u8]) -> Result<Vec<u8>, String> {
    if input.len() < 18 || input[..2] != [0x1f, 0x8b] {
        return Err("invalid gzip header".to_string());
    }
    let mut position = 10;
    let mut output = Vec::new();
    loop {
        let header = *input
            .get(position)
            .ok_or_else(|| "truncated gzip data".to_string())?;
        position += 1;
        if header & 0b110 != 0 {
            return Err("only stored gzip blocks are supported".to_string());
        }
        let length = u16::from_le_bytes([
            *input
                .get(position)
                .ok_or_else(|| "truncated gzip data".to_string())?,
            *input
                .get(position + 1)
                .ok_or_else(|| "truncated gzip data".to_string())?,
        ]) as usize;
        position += 4;
        let end = position + length;
        output.extend_from_slice(
            input
                .get(position..end)
                .ok_or_else(|| "truncated gzip data".to_string())?,
        );
        position = end;
        if header & 1 != 0 {
            break;
        }
    }
    if input.len() < position + 8 {
        return Err("truncated gzip footer".to_string());
    }
    let expected_crc = u32::from_le_bytes(input[position..position + 4].try_into().unwrap());
    if expected_crc != crc32(&output) {
        return Err("gzip checksum mismatch".to_string());
    }
    Ok(output)
}

fn crc32(input: &[u8]) -> u32 {
    let mut crc = u32::MAX;
    for byte in input {
        crc ^= u32::from(*byte);
        for _ in 0..8 {
            crc = if crc & 1 != 0 {
                (crc >> 1) ^ 0xEDB8_8320
            } else {
                crc >> 1
            };
        }
    }
    !crc
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
    use super::{gzip_store, gzip_unstore, part_name, split_bytes, split_lines};

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

    #[test]
    fn round_trips_stored_gzip_parts() {
        let compressed = gzip_store(b"hello compressed part");
        assert_eq!(gzip_unstore(&compressed).unwrap(), b"hello compressed part");
    }
}
