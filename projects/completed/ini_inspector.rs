use std::{collections::BTreeMap, env, fs, path::Path};

fn main() {
    match run(&env::args().skip(1).collect::<Vec<_>>()) {
        Ok(output) => print!("{output}"),
        Err(error) => eprintln!("Error: {error}"),
    }
}

#[derive(Debug, Default, PartialEq, Eq)]
struct Ini {
    sections: BTreeMap<String, BTreeMap<String, String>>,
}

impl Ini {
    fn parse(input: &str) -> Result<Self, String> {
        let mut ini = Self::default();
        let mut section = String::new();
        for (line_number, raw) in input.lines().enumerate() {
            let line = raw.trim();
            if line.is_empty() || line.starts_with('#') || line.starts_with(';') {
                continue;
            }
            if line.starts_with('[') {
                if !line.ends_with(']') || line.len() <= 2 {
                    return Err(format!("line {}: malformed section", line_number + 1));
                }
                section = line[1..line.len() - 1].trim().to_string();
                ini.sections.entry(section.clone()).or_default();
                continue;
            }
            let (key, value) = line
                .split_once('=')
                .ok_or_else(|| format!("line {}: expected key = value", line_number + 1))?;
            let key = key.trim();
            if key.is_empty() {
                return Err(format!("line {}: key cannot be empty", line_number + 1));
            }
            let values = ini.sections.entry(section.clone()).or_default();
            if values.contains_key(key) {
                return Err(format!("line {}: duplicate key '{key}'", line_number + 1));
            }
            values.insert(key.to_string(), value.trim().to_string());
        }
        Ok(ini)
    }

    fn summary(&self) -> String {
        self.sections
            .iter()
            .map(|(section, values)| format!("[{section}] ({} keys)\n", values.len()))
            .collect()
    }

    fn query(&self, path: &str) -> Result<String, String> {
        let (section, key) = path
            .split_once('.')
            .ok_or_else(|| "query must use SECTION.KEY".to_string())?;
        self.sections
            .get(section)
            .and_then(|values| values.get(key))
            .cloned()
            .ok_or_else(|| format!("key '{path}' not found"))
    }

    fn normalized(&self) -> String {
        let mut output = String::new();
        for (section, values) in &self.sections {
            if !section.is_empty() {
                output.push_str(&format!("[{section}]\n"));
            }
            for (key, value) in values {
                output.push_str(&format!("{key} = {value}\n"));
            }
            output.push('\n');
        }
        output
    }
}

fn run(arguments: &[String]) -> Result<String, String> {
    let path = arguments.first().ok_or_else(|| {
        "usage: ini_inspector FILE [--get SECTION.KEY | --rewrite FILE]".to_string()
    })?;
    let ini = Ini::parse(
        &fs::read_to_string(path).map_err(|error| format!("failed to read '{path}': {error}"))?,
    )?;
    match arguments.get(1).map(String::as_str) {
        None => Ok(ini.summary()),
        Some("--get") => ini.query(
            arguments
                .get(2)
                .ok_or_else(|| "--get needs SECTION.KEY".to_string())?,
        ),
        Some("--rewrite") => {
            let output = arguments
                .get(2)
                .ok_or_else(|| "--rewrite needs an output file".to_string())?;
            fs::write(output, ini.normalized()).map_err(|error| {
                format!("failed to write '{}': {error}", Path::new(output).display())
            })?;
            Ok(format!("rewrote {output}\n"))
        }
        Some(option) => Err(format!("unknown option '{option}'")),
    }
}

#[cfg(test)]
mod tests {
    use super::Ini;

    const FIXTURE: &str = "# config\nname = demo\n[server]\nport=8080\nhost = localhost\n";

    #[test]
    fn parses_and_queries_fixture() {
        let ini = Ini::parse(FIXTURE).unwrap();
        assert_eq!(ini.query("server.port").unwrap(), "8080");
        assert!(ini.summary().contains("[server] (2 keys)"));
    }

    #[test]
    fn detects_duplicates_and_malformed_lines() {
        assert!(Ini::parse("[a]\nkey=one\nkey=two").is_err());
        assert!(Ini::parse("[a]\nnot a pair").is_err());
    }

    #[test]
    fn normalizes_order_and_spacing() {
        let ini = Ini::parse("[b]\nz=2\ny=1\n").unwrap();
        assert_eq!(ini.normalized(), "[b]\ny = 1\nz = 2\n\n");
    }
}
