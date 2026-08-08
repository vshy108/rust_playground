use std::{env, fs, path::PathBuf};

fn main() {
    match run(&env::args().skip(1).collect::<Vec<_>>()) {
        Ok(output) => print!("{output}"),
        Err(error) => eprintln!("Error: {error}"),
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Rename {
    from: PathBuf,
    to: PathBuf,
}

struct Options {
    root: PathBuf,
    prefix: String,
    suffix: String,
    replace: Option<(String, String)>,
    numbered: bool,
    apply: bool,
}

fn run(arguments: &[String]) -> Result<String, String> {
    let options = parse_options(arguments)?;
    let plan = plan_renames(&options)?;
    validate_plan(&plan)?;
    let output = plan
        .iter()
        .map(|rename| format!("{} -> {}\n", rename.from.display(), rename.to.display()))
        .collect::<String>();
    if options.apply {
        for rename in &plan {
            fs::rename(&rename.from, &rename.to).map_err(|error| {
                format!("failed to rename '{}': {error}", rename.from.display())
            })?;
        }
    }
    Ok(output)
}

fn parse_options(arguments: &[String]) -> Result<Options, String> {
    let root = arguments.first().ok_or_else(|| "usage: batch_renamer DIRECTORY [--prefix TEXT] [--suffix TEXT] [--replace FROM TO] [--numbered] [--apply]".to_string())?.into();
    let mut options = Options {
        root,
        prefix: String::new(),
        suffix: String::new(),
        replace: None,
        numbered: false,
        apply: false,
    };
    let mut index = 1;
    while index < arguments.len() {
        match arguments[index].as_str() {
            "--prefix" | "--suffix" => {
                let flag = arguments[index].clone();
                index += 1;
                let value = arguments
                    .get(index)
                    .ok_or_else(|| format!("{flag} needs a value"))?
                    .clone();
                if flag == "--prefix" {
                    options.prefix = value;
                } else {
                    options.suffix = value;
                }
            }
            "--replace" => {
                let from = arguments
                    .get(index + 1)
                    .ok_or_else(|| "--replace needs FROM and TO".to_string())?;
                let to = arguments
                    .get(index + 2)
                    .ok_or_else(|| "--replace needs FROM and TO".to_string())?;
                options.replace = Some((from.clone(), to.clone()));
                index += 2;
            }
            "--apply" => options.apply = true,
            "--numbered" => options.numbered = true,
            value => return Err(format!("unknown option '{value}'")),
        }
        index += 1;
    }
    Ok(options)
}

fn plan_renames(options: &Options) -> Result<Vec<Rename>, String> {
    let mut files = fs::read_dir(&options.root)
        .map_err(|error| format!("failed to read '{}': {error}", options.root.display()))?
        .filter_map(Result::ok)
        .map(|entry| entry.path())
        .filter(|path| path.is_file())
        .collect::<Vec<_>>();
    files.sort();
    Ok(files
        .into_iter()
        .enumerate()
        .map(|(index, from)| {
            let name = from
                .file_name()
                .and_then(|value| value.to_str())
                .unwrap_or_default();
            let mut new_name = name.to_string();
            if let Some((from_text, to_text)) = &options.replace {
                new_name = new_name.replace(from_text, to_text);
            }
            new_name = format!("{}{}{}", options.prefix, new_name, options.suffix);
            if options.numbered {
                new_name = format!("{:03}_{}", index + 1, new_name);
            }
            let to = from.with_file_name(new_name);
            Rename { from, to }
        })
        .collect())
}

fn validate_plan(plan: &[Rename]) -> Result<(), String> {
    let mut destinations = std::collections::HashSet::new();
    for rename in plan {
        if rename.from == rename.to {
            return Err(format!(
                "rename for '{}' has no change",
                rename.from.display()
            ));
        }
        if !destinations.insert(&rename.to) {
            return Err(format!("multiple files target '{}'", rename.to.display()));
        }
        if rename.to.exists() && !plan.iter().any(|other| other.from == rename.to) {
            return Err(format!(
                "destination '{}' already exists",
                rename.to.display()
            ));
        }
        if rename
            .to
            .file_name()
            .and_then(|value| value.to_str())
            .is_none_or(str::is_empty)
        {
            return Err("rename target cannot be empty".to_string());
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{Options, Rename, plan_renames, validate_plan};

    #[test]
    fn plans_sorted_prefix_renames_without_mutating() {
        let directory = tempfile::tempdir().unwrap();
        std::fs::write(directory.path().join("b.txt"), b"b").unwrap();
        std::fs::write(directory.path().join("a.txt"), b"a").unwrap();
        let options = Options {
            root: directory.path().to_path_buf(),
            prefix: "new-".to_string(),
            suffix: String::new(),
            replace: None,
            numbered: false,
            apply: false,
        };
        let plan = plan_renames(&options).unwrap();
        assert_eq!(plan[0].from.file_name().unwrap(), "a.txt");
        assert_eq!(plan[0].to.file_name().unwrap(), "new-a.txt");
        assert!(directory.path().join("a.txt").exists());
    }

    #[test]
    fn rejects_colliding_destinations() {
        let directory = tempfile::tempdir().unwrap();
        let plan = vec![
            Rename {
                from: directory.path().join("a.txt"),
                to: directory.path().join("same.txt"),
            },
            Rename {
                from: directory.path().join("b.txt"),
                to: directory.path().join("same.txt"),
            },
        ];
        assert!(validate_plan(&plan).is_err());
    }

    #[test]
    fn prefixes_sorted_files_with_sequential_numbers() {
        let directory = tempfile::tempdir().unwrap();
        std::fs::write(directory.path().join("b.txt"), b"b").unwrap();
        std::fs::write(directory.path().join("a.txt"), b"a").unwrap();
        let options = Options {
            root: directory.path().to_path_buf(),
            prefix: String::new(),
            suffix: String::new(),
            replace: None,
            numbered: true,
            apply: false,
        };
        let plan = plan_renames(&options).unwrap();
        assert_eq!(plan[0].to.file_name().unwrap(), "001_a.txt");
        assert_eq!(plan[1].to.file_name().unwrap(), "002_b.txt");
    }
}
