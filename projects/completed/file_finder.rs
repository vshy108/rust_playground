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

struct Options {
    root: PathBuf,
    name: Option<String>,
    extension: Option<String>,
    max_depth: Option<usize>,
    hidden: bool,
    grep: Option<String>,
}

fn run(arguments: &[String]) -> Result<String, String> {
    let options = parse_options(arguments)?;
    let mut paths = Vec::new();
    walk(&options.root, 0, &options, &mut paths)?;
    paths.sort();
    Ok(paths
        .iter()
        .map(|path| format!("{}\n", path.display()))
        .collect())
}

fn parse_options(arguments: &[String]) -> Result<Options, String> {
    let root = arguments
        .first()
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("."));
    let mut options = Options {
        root,
        name: None,
        extension: None,
        max_depth: None,
        hidden: false,
        grep: None,
    };
    let mut index = 1;
    while index < arguments.len() {
        match arguments[index].as_str() {
            "--name" | "--extension" | "--max-depth" | "--grep" => {
                let flag = arguments[index].clone();
                index += 1;
                let value = arguments
                    .get(index)
                    .ok_or_else(|| format!("{flag} needs a value"))?;
                match flag.as_str() {
                    "--name" => options.name = Some(value.clone()),
                    "--extension" => {
                        options.extension = Some(value.trim_start_matches('.').to_string())
                    }
                    "--max-depth" => {
                        options.max_depth = Some(
                            value
                                .parse()
                                .map_err(|_| "--max-depth must be a number".to_string())?,
                        )
                    }
                    "--grep" => options.grep = Some(value.clone()),
                    _ => unreachable!(),
                }
            }
            "--hidden" => options.hidden = true,
            value => return Err(format!("unknown option '{value}'")),
        }
        index += 1;
    }
    Ok(options)
}

fn walk(
    root: &Path,
    depth: usize,
    options: &Options,
    paths: &mut Vec<PathBuf>,
) -> Result<(), String> {
    if options.max_depth.is_some_and(|limit| depth > limit) {
        return Ok(());
    }
    for entry in
        fs::read_dir(root).map_err(|e| format!("failed to read '{}': {e}", root.display()))?
    {
        let path = entry
            .map_err(|e| format!("failed to read directory entry: {e}"))?
            .path();
        let name = path
            .file_name()
            .and_then(|value| value.to_str())
            .unwrap_or_default();
        if !options.hidden && name.starts_with('.') {
            continue;
        }
        if path.is_dir() {
            walk(&path, depth + 1, options, paths)?;
        } else if matches_file(&path, options)? {
            paths.push(path);
        }
    }
    Ok(())
}

fn matches_file(path: &Path, options: &Options) -> Result<bool, String> {
    let name = path
        .file_name()
        .and_then(|value| value.to_str())
        .unwrap_or_default();
    if !options
        .name
        .as_ref()
        .is_none_or(|pattern| wildcard_match(name, pattern))
    {
        return Ok(false);
    }
    if !options.extension.as_ref().is_none_or(|extension| {
        path.extension().and_then(|value| value.to_str()) == Some(extension)
    }) {
        return Ok(false);
    }
    if let Some(needle) = &options.grep {
        let content = fs::read_to_string(path)
            .map_err(|e| format!("failed to read '{}': {e}", path.display()))?;
        if !content.contains(needle) {
            return Ok(false);
        }
    }
    Ok(true)
}

fn wildcard_match(value: &str, pattern: &str) -> bool {
    if pattern == "*" {
        return true;
    }
    if let Some(suffix) = pattern.strip_prefix('*') {
        return value.ends_with(suffix);
    }
    if let Some(prefix) = pattern.strip_suffix('*') {
        return value.starts_with(prefix);
    }
    value == pattern
}

#[cfg(test)]
mod tests {
    use super::{parse_options, walk, wildcard_match, Options};

    #[test]
    fn matches_simple_wildcards() {
        assert!(wildcard_match("notes.md", "*.md"));
        assert!(wildcard_match("test_notes.rs", "test*"));
        assert!(!wildcard_match("notes.txt", "*.md"));
    }

    #[test]
    fn walks_sorted_filtered_files() {
        let directory = tempfile::tempdir().unwrap();
        std::fs::create_dir(directory.path().join("nested")).unwrap();
        std::fs::write(directory.path().join("b.rs"), b"rust").unwrap();
        std::fs::write(directory.path().join("a.txt"), b"text").unwrap();
        std::fs::write(directory.path().join("nested/c.rs"), b"rust").unwrap();
        let options = Options {
            root: directory.path().to_path_buf(),
            name: None,
            extension: Some("rs".to_string()),
            max_depth: None,
            hidden: false,
            grep: None,
        };
        let mut paths = Vec::new();
        walk(&options.root, 0, &options, &mut paths).unwrap();
        paths.sort();
        assert_eq!(paths.len(), 2);
        assert!(paths[0].ends_with("b.rs"));
        assert!(paths[1].ends_with("c.rs"));
    }

    #[test]
    fn parses_depth_and_hidden_options() {
        let options = parse_options(&[
            ".".to_string(),
            "--hidden".to_string(),
            "--max-depth".to_string(),
            "2".to_string(),
        ])
        .unwrap();
        assert!(options.hidden);
        assert_eq!(options.max_depth, Some(2));
    }
}
