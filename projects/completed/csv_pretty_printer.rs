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
    header: bool,
    columns: Option<Vec<usize>>,
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
    let rows = parse_csv(&input)?;
    let rows = select_columns(rows, options.columns.as_deref())?;
    Ok(render_table(&rows, options.header))
}

fn parse_options(arguments: &[String]) -> Result<Options, String> {
    let mut options = Options {
        header: true,
        columns: None,
        input: None,
    };
    let mut index = 0;
    while index < arguments.len() {
        match arguments[index].as_str() {
            "--no-header" => options.header = false,
            "--columns" => {
                index += 1;
                let value = arguments
                    .get(index)
                    .ok_or_else(|| "--columns needs comma-separated indexes".to_string())?;
                let columns = value
                    .split(',')
                    .map(|column| {
                        column
                            .parse::<usize>()
                            .map_err(|_| format!("'{column}' is not a column index"))
                    })
                    .collect::<Result<Vec<_>, _>>()?;
                if columns.is_empty() {
                    return Err("--columns needs at least one index".to_string());
                }
                options.columns = Some(columns);
            }
            value if value.starts_with('-') => return Err(format!("unknown option '{value}'")),
            path => {
                if options.input.replace(path.to_string()).is_some() {
                    return Err("only one CSV input file is supported".to_string());
                }
            }
        }
        index += 1;
    }
    Ok(options)
}

fn parse_csv(input: &str) -> Result<Vec<Vec<String>>, String> {
    let mut rows = Vec::new();
    let mut row = Vec::new();
    let mut field = String::new();
    let mut quoted = false;
    let mut after_quote = false;
    let mut characters = input.chars().peekable();

    while let Some(character) = characters.next() {
        match (character, quoted, after_quote) {
            ('"', true, false) if characters.peek() == Some(&'"') => {
                characters.next();
                field.push('"');
            }
            ('"', true, false) => {
                quoted = false;
                after_quote = true;
            }
            ('"', false, false) if field.is_empty() => quoted = true,
            (',', false, false) => {
                row.push(std::mem::take(&mut field));
            }
            ('\n', false, false) => {
                row.push(std::mem::take(&mut field));
                if row.last().is_some_and(|value| value.ends_with('\r')) {
                    row.last_mut().unwrap().pop();
                }
                if !row.is_empty() {
                    rows.push(std::mem::take(&mut row));
                }
            }
            ('\r', false, false) if characters.peek() == Some(&'\n') => {}
            (character, true, false) => field.push(character),
            (character, false, true) if character == ' ' || character == '\t' => {}
            (',', false, true) => {
                row.push(std::mem::take(&mut field));
                after_quote = false;
            }
            ('\n', false, true) => {
                row.push(std::mem::take(&mut field));
                rows.push(std::mem::take(&mut row));
                after_quote = false;
            }
            (character, false, true) => {
                return Err(format!(
                    "unexpected character '{character}' after closing quote"
                ))
            }
            (character, true, true) => {
                return Err(format!(
                    "unexpected character '{character}' after closing quote"
                ))
            }
            (character, false, false) => field.push(character),
        }
    }

    if quoted {
        return Err("unterminated quoted CSV field".to_string());
    }
    if after_quote || !field.is_empty() || !row.is_empty() {
        row.push(field);
        rows.push(row);
    }
    Ok(rows)
}

fn select_columns(
    rows: Vec<Vec<String>>,
    columns: Option<&[usize]>,
) -> Result<Vec<Vec<String>>, String> {
    let Some(columns) = columns else {
        return Ok(rows);
    };
    rows.into_iter()
        .map(|row| {
            columns
                .iter()
                .map(|&index| {
                    row.get(index)
                        .cloned()
                        .ok_or_else(|| format!("column {index} is missing from a row"))
                })
                .collect()
        })
        .collect()
}

fn render_table(rows: &[Vec<String>], header: bool) -> String {
    if rows.is_empty() {
        return String::new();
    }
    let width = rows.iter().map(Vec::len).max().unwrap_or(0);
    let mut normalized = rows.to_vec();
    for row in &mut normalized {
        row.resize(width, String::new());
    }
    let widths = (0..width)
        .map(|column| {
            normalized
                .iter()
                .map(|row| row[column].len())
                .max()
                .unwrap_or(0)
        })
        .collect::<Vec<_>>();
    let render_row = |row: &[String]| {
        row.iter()
            .enumerate()
            .map(|(index, value)| format!(" {:width$} ", value, width = widths[index]))
            .collect::<Vec<_>>()
            .join("|")
    };
    let mut output = String::new();
    if header {
        output.push_str(&render_row(&normalized[0]));
        output.push('\n');
        output.push_str(
            &widths
                .iter()
                .map(|width| "-".repeat(width + 2))
                .collect::<Vec<_>>()
                .join("+"),
        );
        output.push('\n');
        for row in &normalized[1..] {
            output.push_str(&render_row(row));
            output.push('\n');
        }
    } else {
        for row in &normalized {
            output.push_str(&render_row(row));
            output.push('\n');
        }
    }
    output
}

#[cfg(test)]
mod tests {
    use super::{parse_csv, render_table, select_columns};

    #[test]
    fn parses_quoted_commas_and_escaped_quotes() {
        assert_eq!(
            parse_csv("name,note\nAda,\"likes, Rust\"\n\"Grace \"\"G\"\" Hopper\",compiler\n")
                .unwrap(),
            vec![
                vec!["name", "note"],
                vec!["Ada", "likes, Rust"],
                vec!["Grace \"G\" Hopper", "compiler"],
            ]
        );
    }

    #[test]
    fn renders_aligned_header_table() {
        let rows = parse_csv("name,age\nAda,36\nBob,7\n").unwrap();
        assert_eq!(
            render_table(&rows, true),
            " name | age \n------+-----\n Ada  | 36  \n Bob  | 7   \n"
        );
    }

    #[test]
    fn selects_columns_and_rejects_missing_values() {
        let rows = parse_csv("a,b\n1,2\n").unwrap();
        assert_eq!(
            select_columns(rows, Some(&[1])).unwrap(),
            vec![vec!["b"], vec!["2"]]
        );
        assert!(select_columns(vec![vec!["a".to_string()]], Some(&[1])).is_err());
    }

    #[test]
    fn rejects_unterminated_quotes() {
        assert!(parse_csv("name,\"missing\n").is_err());
    }
}
