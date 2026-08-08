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

struct Options {
    compact: bool,
    path: Option<String>,
    input: Option<String>,
}

fn run(arguments: &[String]) -> Result<String, String> {
    let options = parse_options(arguments)?;
    let input = match options.input.as_deref() {
        Some(path) => {
            fs::read_to_string(path).map_err(|error| format!("failed to read '{path}': {error}"))?
        }
        None => {
            let mut input = String::new();
            io::stdin()
                .read_to_string(&mut input)
                .map_err(|error| format!("failed to read standard input: {error}"))?;
            input
        }
    };
    let value = parse_json(&input)?;
    let value = match options.path.as_deref() {
        Some(path) => lookup_path(&value, path)?,
        None => &value,
    };
    let output = if options.compact {
        serde_json::to_string(value)
    } else {
        serde_json::to_string_pretty(value)
    }
    .map_err(|error| format!("failed to render JSON: {error}"))?;
    Ok(format!("{output}\n"))
}

fn parse_json(input: &str) -> Result<serde_json::Value, String> {
    serde_json::from_str(input).map_err(|error| {
        format!(
            "failed to parse JSON at line {}, column {}: {error}",
            error.line(),
            error.column()
        )
    })
}

fn parse_options(arguments: &[String]) -> Result<Options, String> {
    let mut options = Options {
        compact: false,
        path: None,
        input: None,
    };
    let mut index = 0;
    while index < arguments.len() {
        match arguments[index].as_str() {
            "--compact" => options.compact = true,
            "--get" => {
                index += 1;
                let path = arguments
                    .get(index)
                    .ok_or_else(|| "--get needs a dotted path".to_string())?;
                options.path = Some(path.clone());
            }
            value if value.starts_with('-') => return Err(format!("unknown option '{value}'")),
            path => {
                if options.input.replace(path.to_string()).is_some() {
                    return Err("only one JSON input file is supported".to_string());
                }
            }
        }
        index += 1;
    }
    Ok(options)
}

fn lookup_path<'a>(
    value: &'a serde_json::Value,
    path: &str,
) -> Result<&'a serde_json::Value, String> {
    if path.is_empty() {
        return Err("JSON lookup path cannot be empty".to_string());
    }
    path.split('.').try_fold(value, |current, segment| {
        if segment.is_empty() {
            return Err("JSON lookup path contains an empty segment".to_string());
        }
        match current {
            serde_json::Value::Object(object) => object
                .get(segment)
                .ok_or_else(|| format!("JSON path '{path}' was not found")),
            serde_json::Value::Array(array) => {
                let index = segment
                    .parse::<usize>()
                    .map_err(|_| format!("JSON path segment '{segment}' is not an array index"))?;
                array
                    .get(index)
                    .ok_or_else(|| format!("JSON path '{path}' was not found"))
            }
            _ => Err(format!("JSON path '{path}' cannot descend into a scalar")),
        }
    })
}

#[cfg(test)]
mod tests {
    use super::{lookup_path, parse_json, parse_options};

    #[test]
    fn parses_rendering_and_lookup_options() {
        let options = parse_options(&[
            "--compact".to_string(),
            "--get".to_string(),
            "users.0.name".to_string(),
            "sample.json".to_string(),
        ])
        .unwrap();
        assert!(options.compact);
        assert_eq!(options.path.as_deref(), Some("users.0.name"));
        assert_eq!(options.input.as_deref(), Some("sample.json"));
    }

    #[test]
    fn looks_up_nested_objects_and_arrays() {
        let value: serde_json::Value = serde_json::json!({
            "users": [{"name": "Ada"}]
        });
        assert_eq!(lookup_path(&value, "users.0.name").unwrap(), "Ada");
    }

    #[test]
    fn reports_missing_paths() {
        let value = serde_json::json!({"name": "Ada"});
        assert!(lookup_path(&value, "missing").is_err());
    }

    #[test]
    fn reports_json_parse_location() {
        let error = parse_json("{\"name\":").unwrap_err();
        assert!(error.contains("line 1") && error.contains("column"));
    }
}
