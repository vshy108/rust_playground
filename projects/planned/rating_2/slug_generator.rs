// Solution flow:
// 1. Read text from command-line arguments, or from standard input when no text was supplied.
// 2. Convert ASCII letters to lowercase and treat every non-alphanumeric character as a separator.
// 3. Emit at most one `-` for consecutive separators and trim it from both ends.
// 4. Optionally shorten the finished slug, then print the predictable result.
use std::{
    env,
    io::{self, Read},
};

fn main() {
    match run(&env::args().skip(1).collect::<Vec<_>>()) {
        Ok(slug) => println!("{slug}"),
        Err(error) => eprintln!("Error: {error}"),
    }
}

fn run(arguments: &[String]) -> Result<String, String> {
    let (maximum_length, text_parts) = parse_arguments(arguments)?;
    let input = if text_parts.is_empty() {
        read_stdin()?
    } else {
        text_parts.join(" ")
    };

    Ok(slugify(&input, maximum_length))
}

fn parse_arguments(arguments: &[String]) -> Result<(Option<usize>, Vec<String>), String> {
    let mut maximum_length = None;
    let mut text = Vec::new();
    let mut index = 0;

    while index < arguments.len() {
        if arguments[index] == "--max-length" {
            index += 1;
            let value = arguments
                .get(index)
                .ok_or_else(|| "--max-length needs a positive number".to_string())?;
            let length = value
                .parse::<usize>()
                .map_err(|_| "--max-length must be a positive number".to_string())?;
            if length == 0 {
                return Err("--max-length must be greater than zero".to_string());
            }
            maximum_length = Some(length);
        } else {
            text.push(arguments[index].clone());
        }
        index += 1;
    }

    Ok((maximum_length, text))
}

fn read_stdin() -> Result<String, String> {
    let mut input = String::new();
    io::stdin()
        .read_to_string(&mut input)
        .map_err(|error| format!("failed to read standard input: {error}"))?;
    Ok(input)
}

fn slugify(input: &str, maximum_length: Option<usize>) -> String {
    let mut slug = String::new();
    let mut previous_was_separator = true;

    for character in input.chars() {
        if character.is_ascii_alphanumeric() {
            slug.push(character.to_ascii_lowercase());
            previous_was_separator = false;
        } else if !previous_was_separator {
            slug.push('-');
            previous_was_separator = true;
        }
    }

    let slug = slug.trim_matches('-');
    let limited = match maximum_length {
        Some(length) => slug.chars().take(length).collect::<String>(),
        None => slug.to_string(),
    };

    limited.trim_matches('-').to_string()
}

#[cfg(test)]
mod tests {
    use super::{parse_arguments, slugify};

    #[test]
    fn normalizes_punctuation_and_whitespace() {
        assert_eq!(
            slugify("  Rust: Fast, Safe & Fun!  ", None),
            "rust-fast-safe-fun"
        );
    }

    #[test]
    fn collapses_and_trims_separators() {
        assert_eq!(slugify("---one___two...three---", None), "one-two-three");
    }

    #[test]
    fn uses_ascii_only_for_predictable_slugs() {
        assert_eq!(slugify("Café déjà vu", None), "caf-d-j-vu");
    }

    #[test]
    fn limits_finished_slugs_without_a_trailing_separator() {
        assert_eq!(slugify("One Two Three", Some(8)), "one-two");
    }

    #[test]
    fn validates_maximum_length() {
        let arguments = ["--max-length".to_string(), "0".to_string()];
        assert!(parse_arguments(&arguments).is_err());
    }
}
