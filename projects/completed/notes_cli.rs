use std::{env, fs, path::Path};

fn main() {
    match run(&env::args().skip(1).collect::<Vec<_>>()) {
        Ok(output) => print!("{output}"),
        Err(error) => eprintln!("Error: {error}"),
    }
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
struct Note {
    id: u64,
    text: String,
    tags: Vec<String>,
    completed: bool,
}

fn run(arguments: &[String]) -> Result<String, String> {
    let (path, command) = parse_options(arguments)?;
    let mut notes = load_notes(&path)?;
    let output = match command {
        Command::Add { text, tags } => {
            let id = notes.iter().map(|note| note.id).max().unwrap_or(0) + 1;
            notes.push(Note {
                id,
                text,
                tags,
                completed: false,
            });
            save_notes(&path, &notes)?;
            format!("added note {id}\n")
        }
        Command::List { tag } => render_notes(&notes, tag.as_deref()),
        Command::Complete(id) => update_note(&mut notes, id, |note| note.completed = true, &path)?,
        Command::Delete(id) => {
            let before = notes.len();
            notes.retain(|note| note.id != id);
            if notes.len() == before {
                return Err(format!("note {id} was not found"));
            }
            save_notes(&path, &notes)?;
            format!("deleted note {id}\n")
        }
    };
    Ok(output)
}

enum Command {
    Add { text: String, tags: Vec<String> },
    List { tag: Option<String> },
    Complete(u64),
    Delete(u64),
}

fn parse_options(arguments: &[String]) -> Result<(String, Command), String> {
    let mut index = 0;
    let mut path = ".notes.json".to_string();
    if arguments.get(index).map(String::as_str) == Some("--file") {
        index += 1;
        path = arguments
            .get(index)
            .ok_or_else(|| "--file needs a path".to_string())?
            .clone();
        index += 1;
    }
    let command = match arguments.get(index).map(String::as_str) {
        Some("add") => {
            let text_start = index + 1;
            if text_start >= arguments.len() { return Err("add needs note text".to_string()); }
            let mut tags = Vec::new();
            let mut text = Vec::new();
            for value in &arguments[text_start..] {
                if let Some(tag) = value.strip_prefix("tag:") { tags.push(tag.to_string()); } else { text.push(value.clone()); }
            }
            Command::Add { text: text.join(" "), tags }
        }
        Some("list") => {
            let tag = arguments.get(index + 1).map(|value| value.strip_prefix("tag:").unwrap_or(value).to_string());
            Command::List { tag }
        }
        Some("complete") => Command::Complete(parse_id(arguments.get(index + 1))?),
        Some("delete") => Command::Delete(parse_id(arguments.get(index + 1))?),
        _ => return Err("usage: notes_cli [--file PATH] add TEXT [tag:NAME] | list [TAG] | complete ID | delete ID".to_string()),
    };
    Ok((path, command))
}

fn parse_id(value: Option<&String>) -> Result<u64, String> {
    value
        .ok_or_else(|| "command needs a note id".to_string())?
        .parse()
        .map_err(|_| "note id must be a number".to_string())
}

fn load_notes(path: &str) -> Result<Vec<Note>, String> {
    if !Path::new(path).exists() {
        return Ok(Vec::new());
    }
    let content = fs::read_to_string(path).map_err(|e| format!("failed to read '{path}': {e}"))?;
    serde_json::from_str(&content).map_err(|e| format!("failed to parse '{path}': {e}"))
}

fn save_notes(path: &str, notes: &[Note]) -> Result<(), String> {
    let content =
        serde_json::to_string_pretty(notes).map_err(|e| format!("failed to encode notes: {e}"))?;
    fs::write(path, format!("{content}\n")).map_err(|e| format!("failed to write '{path}': {e}"))
}

fn update_note(
    path_notes: &mut [Note],
    id: u64,
    update: impl FnOnce(&mut Note),
    path: &str,
) -> Result<String, String> {
    let note = path_notes
        .iter_mut()
        .find(|note| note.id == id)
        .ok_or_else(|| format!("note {id} was not found"))?;
    update(note);
    save_notes(path, path_notes)?;
    Ok(format!("updated note {id}\n"))
}

fn render_notes(notes: &[Note], tag: Option<&str>) -> String {
    notes
        .iter()
        .filter(|note| tag.is_none_or(|tag| note.tags.iter().any(|value| value == tag)))
        .map(|note| {
            let status = if note.completed { "x" } else { " " };
            let tags = if note.tags.is_empty() {
                String::new()
            } else {
                format!(" [{}]", note.tags.join(","))
            };
            format!("{} [{}] {}{}\n", note.id, status, note.text, tags)
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::{load_notes, render_notes, save_notes, Note};

    #[test]
    fn saves_and_loads_notes_with_tags() {
        let directory = tempfile::tempdir().unwrap();
        let path = directory.path().join("notes.json");
        let notes = vec![Note {
            id: 1,
            text: "Learn Rust".to_string(),
            tags: vec!["study".to_string()],
            completed: false,
        }];
        save_notes(path.to_str().unwrap(), &notes).unwrap();
        assert_eq!(load_notes(path.to_str().unwrap()).unwrap(), notes);
    }

    #[test]
    fn renders_completion_and_tag_filtering() {
        let notes = vec![
            Note {
                id: 1,
                text: "Done".to_string(),
                tags: vec!["work".to_string()],
                completed: true,
            },
            Note {
                id: 2,
                text: "Later".to_string(),
                tags: vec!["home".to_string()],
                completed: false,
            },
        ];
        assert_eq!(render_notes(&notes, Some("work")), "1 [x] Done [work]\n");
    }
}
