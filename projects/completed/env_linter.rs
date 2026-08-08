use std::{
    collections::HashSet,
    env, fs,
    io::{self, Read},
};

fn main() {
    match run(&env::args().skip(1).collect::<Vec<_>>()) {
        Ok(output) => print!("{output}"),
        Err(error) => eprintln!("Error: {error}"),
    }
}

#[derive(Debug, PartialEq, Eq, serde::Serialize)]
struct Issue {
    line: usize,
    code: &'static str,
    message: String,
    suggestion: Option<String>,
}

fn run(arguments: &[String]) -> Result<String, String> {
    let (json, input) = match arguments {
        [] => (false, read_stdin()?),
        [flag] if flag == "--json" => (true, read_stdin()?),
        [path] => (
            false,
            fs::read_to_string(path).map_err(|e| format!("failed to read '{path}': {e}"))?,
        ),
        [flag, path] if flag == "--json" => (
            true,
            fs::read_to_string(path).map_err(|e| format!("failed to read '{path}': {e}"))?,
        ),
        _ => return Err("usage: env_linter [--json] [FILE]".to_string()),
    };
    let issues = lint(&input);
    if json {
        serde_json::to_string_pretty(&issues)
            .map(|value| format!("{value}\n"))
            .map_err(|e| e.to_string())
    } else {
        Ok(render_human(&issues))
    }
}

fn read_stdin() -> Result<String, String> {
    let mut input = String::new();
    io::stdin()
        .read_to_string(&mut input)
        .map_err(|e| format!("failed to read standard input: {e}"))?;
    Ok(input)
}

fn lint(input: &str) -> Vec<Issue> {
    let mut seen = HashSet::new();
    let mut issues = Vec::new();
    for (index, raw) in input.lines().enumerate() {
        let line = index + 1;
        let trimmed = raw.trim();
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }
        let Some((raw_key, raw_value)) = raw.split_once('=') else {
            issues.push(Issue {
                line,
                code: "E001",
                message: "line must contain '='".to_string(),
                suggestion: None,
            });
            continue;
        };
        let key = raw_key.trim();
        let value = raw_value.trim();
        if key != raw_key || value != raw_value {
            issues.push(Issue {
                line,
                code: "W001",
                message: "unnecessary whitespace around assignment".to_string(),
                suggestion: Some(format!("{key}={value}")),
            });
        }
        if !valid_name(key) {
            issues.push(Issue {
                line,
                code: "E002",
                message: format!("invalid environment variable name '{key}'"),
                suggestion: None,
            });
        }
        if !seen.insert(key.to_string()) {
            issues.push(Issue {
                line,
                code: "E003",
                message: format!("duplicate key '{key}'"),
                suggestion: None,
            });
        }
        if value.is_empty() {
            issues.push(Issue {
                line,
                code: "W002",
                message: format!("key '{key}' has an empty value"),
                suggestion: Some(format!("{key}=<value>")),
            });
        }
    }
    issues
}

fn valid_name(name: &str) -> bool {
    let mut chars = name.chars();
    matches!(chars.next(), Some(first) if first == '_' || first.is_ascii_alphabetic())
        && chars.all(|character| character == '_' || character.is_ascii_alphanumeric())
}

fn render_human(issues: &[Issue]) -> String {
    if issues.is_empty() {
        return "No issues found.\n".to_string();
    }
    issues
        .iter()
        .map(|issue| {
            let suggestion = issue
                .suggestion
                .as_deref()
                .map(|value| format!(" (suggestion: {value})"))
                .unwrap_or_default();
            format!(
                "line {} [{}] {}{}\n",
                issue.line, issue.code, issue.message, suggestion
            )
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::{lint, render_human, valid_name};

    #[test]
    fn reports_duplicate_invalid_and_empty_values() {
        let issues = lint("GOOD=value\nGOOD=again\n9BAD=x\nEMPTY=\n");
        assert!(issues.iter().any(|issue| issue.code == "E003"));
        assert!(issues.iter().any(|issue| issue.code == "E002"));
        assert!(issues.iter().any(|issue| issue.code == "W002"));
    }

    #[test]
    fn offers_whitespace_autofix_suggestions() {
        let issues = lint(" KEY = value \n");
        assert_eq!(issues[0].suggestion.as_deref(), Some("KEY=value"));
        assert!(render_human(&issues).contains("suggestion: KEY=value"));
    }

    #[test]
    fn ignores_comments_and_validates_names() {
        assert!(lint("# comment\nGOOD=value\n").is_empty());
        assert!(valid_name("_PRIVATE_2"));
        assert!(!valid_name("bad-name"));
    }
}
