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
    if arguments.len() != 2 {
        return Err("usage: text_diff_cli LEFT_FILE_OR_- RIGHT_FILE_OR_-".to_string());
    }
    let left = read_input(&arguments[0])?;
    let right = read_input(&arguments[1])?;
    Ok(render_diff(&lines(&left), &lines(&right)))
}

fn read_input(path: &str) -> Result<String, String> {
    if path == "-" {
        let mut input = String::new();
        io::stdin()
            .read_to_string(&mut input)
            .map_err(|error| format!("failed to read stdin: {error}"))?;
        Ok(input)
    } else {
        fs::read_to_string(path).map_err(|error| format!("failed to read '{path}': {error}"))
    }
}

fn lines(input: &str) -> Vec<&str> {
    input.lines().collect()
}

#[derive(Debug, PartialEq, Eq)]
enum Change<'a> {
    Same(&'a str),
    Added(&'a str),
    Removed(&'a str),
}

fn diff<'a>(left: &[&'a str], right: &[&'a str]) -> Vec<Change<'a>> {
    let mut lengths = vec![vec![0usize; right.len() + 1]; left.len() + 1];
    for i in (0..left.len()).rev() {
        for j in (0..right.len()).rev() {
            lengths[i][j] = if left[i] == right[j] {
                lengths[i + 1][j + 1] + 1
            } else {
                lengths[i + 1][j].max(lengths[i][j + 1])
            };
        }
    }
    let (mut i, mut j) = (0, 0);
    let mut changes = Vec::new();
    while i < left.len() && j < right.len() {
        if left[i] == right[j] {
            changes.push(Change::Same(left[i]));
            i += 1;
            j += 1;
        } else if lengths[i + 1][j] >= lengths[i][j + 1] {
            changes.push(Change::Removed(left[i]));
            i += 1;
        } else {
            changes.push(Change::Added(right[j]));
            j += 1;
        }
    }
    while i < left.len() {
        changes.push(Change::Removed(left[i]));
        i += 1;
    }
    while j < right.len() {
        changes.push(Change::Added(right[j]));
        j += 1;
    }
    changes
}

fn render_diff(left: &[&str], right: &[&str]) -> String {
    let changes = diff(left, right);
    let mut output = String::new();
    let mut index = 0;
    while index < changes.len() {
        match (&changes[index], changes.get(index + 1)) {
            (Change::Removed(old), Some(Change::Added(new))) => {
                output.push_str(&format!(
                    "- {}\n+ {}\n",
                    highlight(old, new, '-'),
                    highlight(new, old, '+')
                ));
                index += 2;
            }
            (Change::Same(line), _) => {
                output.push_str(&format!("  {line}\n"));
                index += 1;
            }
            (Change::Removed(line), _) => {
                output.push_str(&format!("- {line}\n"));
                index += 1;
            }
            (Change::Added(line), _) => {
                output.push_str(&format!("+ {line}\n"));
                index += 1;
            }
        }
    }
    output
}

fn highlight(primary: &str, other: &str, marker: char) -> String {
    let primary_words = primary.split_whitespace().collect::<Vec<_>>();
    let other_words = other.split_whitespace().collect::<Vec<_>>();
    if primary_words == other_words {
        return primary.to_string();
    }
    let mut output = String::new();
    for (index, word) in primary_words.iter().enumerate() {
        if index > 0 {
            output.push(' ');
        }
        if other_words.get(index) != Some(word) {
            output.push(marker);
            output.push_str(word);
            output.push(marker);
        } else {
            output.push_str(word);
        }
    }
    output
}

#[cfg(test)]
mod tests {
    use super::{Change, diff, lines, render_diff};

    #[test]
    fn renders_added_removed_and_same_lines() {
        let left = lines("keep\nold\n");
        let right = lines("keep\nnew\nextra\n");
        assert_eq!(
            render_diff(&left, &right),
            "  keep\n- -old-\n+ +new+\n+ extra\n"
        );
    }

    #[test]
    fn handles_empty_inputs() {
        assert_eq!(render_diff(&[], &lines("one\n")), "+ one\n");
        assert_eq!(render_diff(&lines("one\n"), &[]), "- one\n");
    }

    #[test]
    fn computes_stable_lcs_changes() {
        assert_eq!(
            diff(&lines("a\nb"), &lines("a\nc")),
            vec![Change::Same("a"), Change::Removed("b"), Change::Added("c")]
        );
    }
}
