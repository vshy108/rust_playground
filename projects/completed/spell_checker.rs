use std::{collections::BTreeMap, env, fs};

fn main() {
    match run(&env::args().skip(1).collect::<Vec<_>>()) {
        Ok(output) => print!("{output}"),
        Err(error) => eprintln!("Error: {error}"),
    }
}

fn run(args: &[String]) -> Result<String, String> {
    let dictionary = load_dictionary(args.first().ok_or_else(|| {
        "usage: spell_checker DICTIONARY TEXT_FILE [--autocomplete PREFIX]".to_string()
    })?)?;
    let input = fs::read_to_string(
        args.get(1)
            .ok_or_else(|| "TEXT_FILE is required".to_string())?,
    )
    .map_err(|error| error.to_string())?;
    if args.get(2).map(String::as_str) == Some("--autocomplete") {
        let prefix = args
            .get(3)
            .ok_or_else(|| "--autocomplete needs a prefix".to_string())?;
        return Ok(autocomplete(&dictionary, &normalize(prefix)).join("\n") + "\n");
    }
    let mut output = String::new();
    for word in tokenize(&input) {
        if !dictionary.contains_key(&word) {
            let suggestions = suggest(&dictionary, &word, 3);
            output.push_str(&format!("{word}: {}\n", suggestions.join(", ")));
        }
    }
    Ok(output)
}

fn normalize(value: &str) -> String {
    value
        .chars()
        .filter(|character| character.is_ascii_alphabetic())
        .flat_map(char::to_lowercase)
        .collect()
}

fn tokenize(input: &str) -> Vec<String> {
    input
        .split_whitespace()
        .map(normalize)
        .filter(|word| !word.is_empty())
        .collect()
}

fn load_dictionary(path: &str) -> Result<BTreeMap<String, u64>, String> {
    let text =
        fs::read_to_string(path).map_err(|error| format!("failed to read dictionary: {error}"))?;
    let mut dictionary = BTreeMap::new();
    for line in text
        .lines()
        .filter(|line| !line.trim().is_empty() && !line.trim_start().starts_with('#'))
    {
        let mut fields = line.split_whitespace();
        let word = normalize(
            fields
                .next()
                .ok_or_else(|| "dictionary entry has no word".to_string())?,
        );
        let frequency = fields
            .next()
            .unwrap_or("1")
            .parse()
            .map_err(|_| format!("invalid frequency for '{word}'"))?;
        dictionary.insert(word, frequency);
    }
    Ok(dictionary)
}

fn distance(left: &str, right: &str) -> usize {
    let mut row = (0..=right.chars().count()).collect::<Vec<_>>();
    for (i, a) in left.chars().enumerate() {
        let mut next = vec![i + 1];
        for (j, b) in right.chars().enumerate() {
            next.push(if a == b {
                row[j]
            } else {
                1 + row[j].min(row[j + 1]).min(next[j])
            });
        }
        row = next;
    }
    row[right.chars().count()]
}

fn suggest(dictionary: &BTreeMap<String, u64>, word: &str, limit: usize) -> Vec<String> {
    let mut candidates = dictionary
        .iter()
        .map(|(candidate, frequency)| (distance(word, candidate), *frequency, candidate))
        .filter(|(distance, _, _)| *distance <= 2)
        .collect::<Vec<_>>();
    candidates.sort_by(|left, right| {
        left.0
            .cmp(&right.0)
            .then_with(|| right.1.cmp(&left.1))
            .then_with(|| left.2.cmp(right.2))
    });
    candidates
        .into_iter()
        .take(limit)
        .map(|(_, _, word)| word.clone())
        .collect()
}

fn autocomplete(dictionary: &BTreeMap<String, u64>, prefix: &str) -> Vec<String> {
    let mut words = dictionary
        .iter()
        .filter(|(word, _)| word.starts_with(prefix))
        .map(|(word, frequency)| (word, *frequency))
        .collect::<Vec<_>>();
    words.sort_by(|left, right| right.1.cmp(&left.1).then_with(|| left.0.cmp(right.0)));
    words.into_iter().map(|(word, _)| word.clone()).collect()
}

#[cfg(test)]
mod tests {
    use super::{distance, normalize, suggest};
    use std::collections::BTreeMap;

    #[test]
    fn normalizes_and_measures_distance() {
        assert_eq!(normalize("Café!"), "caf");
        assert_eq!(distance("kitten", "sitting"), 3);
    }

    #[test]
    fn ranks_close_candidates_by_distance_then_frequency() {
        let dictionary = BTreeMap::from([("cat".into(), 2), ("cut".into(), 9), ("dog".into(), 1)]);
        assert_eq!(suggest(&dictionary, "cot", 2), vec!["cut", "cat"]);
    }
}
