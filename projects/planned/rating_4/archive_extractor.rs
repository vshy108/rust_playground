use std::{
    env, fs,
    path::{Component, Path, PathBuf},
};

fn main() {
    match run(&env::args().skip(1).collect::<Vec<_>>()) {
        Ok(output) => print!("{output}"),
        Err(error) => eprintln!("Error: {error}"),
    }
}

struct Entry {
    path: PathBuf,
    data: Vec<u8>,
}

fn run(arguments: &[String]) -> Result<String, String> {
    let (mode, archive, target) = match arguments {
        [mode, archive] if mode == "list" => (mode.as_str(), archive.as_str(), None),
        [mode, archive, target] if mode == "extract" => {
            (mode.as_str(), archive.as_str(), Some(target.as_str()))
        }
        _ => {
            return Err(
                "usage: archive_extractor list ARCHIVE | extract ARCHIVE DIRECTORY".to_string(),
            );
        }
    };
    let bytes =
        fs::read(archive).map_err(|error| format!("failed to read '{archive}': {error}"))?;
    let entries = parse_tar(&bytes)?;
    match mode {
        "list" => Ok(entries
            .iter()
            .map(|entry| format!("{}\n", entry.path.display()))
            .collect()),
        "extract" => {
            let target = Path::new(target.unwrap());
            for entry in &entries {
                let destination = target.join(&entry.path);
                if entry.path.extension().is_none() && entry.data.is_empty() {
                    fs::create_dir_all(&destination).map_err(|error| {
                        format!("failed to create '{}': {error}", destination.display())
                    })?;
                } else {
                    if let Some(parent) = destination.parent() {
                        fs::create_dir_all(parent).map_err(|error| {
                            format!("failed to create '{}': {error}", parent.display())
                        })?;
                    }
                    fs::write(&destination, &entry.data).map_err(|error| {
                        format!("failed to write '{}': {error}", destination.display())
                    })?;
                }
            }
            Ok(format!(
                "extracted {} entr{}\n",
                entries.len(),
                if entries.len() == 1 { "y" } else { "ies" }
            ))
        }
        _ => unreachable!(),
    }
}

fn parse_tar(bytes: &[u8]) -> Result<Vec<Entry>, String> {
    let mut entries = Vec::new();
    let mut offset = 0;
    while offset + 512 <= bytes.len() {
        let header = &bytes[offset..offset + 512];
        if header.iter().all(|byte| *byte == 0) {
            break;
        }
        let name = field_string(&header[..100])?;
        let prefix = field_string(&header[345..500])?;
        let path = if prefix.is_empty() {
            name
        } else {
            format!("{prefix}/{name}")
        };
        let path = safe_path(&path)?;
        let size = parse_octal(&header[124..136])?;
        let data_start = offset + 512;
        let data_end = data_start
            .checked_add(size)
            .ok_or_else(|| "archive entry is too large".to_string())?;
        if data_end > bytes.len() {
            return Err(format!("entry '{}' extends past archive", path.display()));
        }
        let data = bytes[data_start..data_end].to_vec();
        entries.push(Entry { path, data });
        offset = data_start + size.div_ceil(512) * 512;
    }
    Ok(entries)
}

fn field_string(field: &[u8]) -> Result<String, String> {
    let end = field
        .iter()
        .position(|byte| *byte == 0)
        .unwrap_or(field.len());
    std::str::from_utf8(&field[..end])
        .map(str::to_string)
        .map_err(|_| "archive header contains invalid UTF-8".to_string())
}

fn parse_octal(field: &[u8]) -> Result<usize, String> {
    let text = field_string(field)?
        .trim_matches(char::from(0))
        .trim()
        .to_string();
    if text.is_empty() {
        return Ok(0);
    }
    usize::from_str_radix(&text, 8).map_err(|_| format!("invalid tar size '{text}'"))
}

fn safe_path(path: &str) -> Result<PathBuf, String> {
    let path = Path::new(path);
    if path.is_absolute()
        || path
            .components()
            .any(|component| matches!(component, Component::ParentDir | Component::Prefix(_)))
    {
        return Err(format!("unsafe archive path '{}'", path.display()));
    }
    Ok(path.to_path_buf())
}

#[cfg(test)]
mod tests {
    use super::{parse_tar, safe_path};

    fn tar_entry(path: &str, data: &[u8]) -> Vec<u8> {
        let mut header = [0u8; 512];
        header[..path.len()].copy_from_slice(path.as_bytes());
        let size = format!("{:011o}\0", data.len());
        header[124..136].copy_from_slice(size.as_bytes());
        header[156] = b'0';
        let mut archive = header.to_vec();
        archive.extend_from_slice(data);
        archive.resize(archive.len().next_multiple_of(512), 0);
        archive.extend_from_slice(&[0u8; 1024]);
        archive
    }

    #[test]
    fn reads_tar_entries_and_preserves_data() {
        let archive = tar_entry("nested/hello.txt", b"hello");
        let entries = parse_tar(&archive).unwrap();
        assert_eq!(entries[0].path.to_str(), Some("nested/hello.txt"));
        assert_eq!(entries[0].data, b"hello");
    }

    #[test]
    fn rejects_path_traversal() {
        assert!(safe_path("../outside.txt").is_err());
        assert!(safe_path("/absolute.txt").is_err());
        assert!(safe_path("safe/file.txt").is_ok());
    }
}
