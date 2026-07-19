// Solution flow:
// 1. Select `encode`, `decode`, or `query` mode from the first command-line argument.
// 2. Read the remaining text from arguments, or standard input when it is omitted.
// 3. Encode only URL-component-safe bytes, or validate and decode every `%HH` escape sequence.
// 4. Print the transformed component or query string; malformed escapes become clear errors.
use std::{
    env,
    io::{self, Read},
};

fn main() {
    match run(&env::args().skip(1).collect::<Vec<_>>()) {
        Ok(output) => println!("{output}"),
        Err(error) => eprintln!("Error: {error}"),
    }
}

fn run(arguments: &[String]) -> Result<String, String> {
    let (mode, values) = arguments
        .split_first()
        .ok_or_else(|| "usage: url_codec <encode|decode|query> [text]".to_string())?;

    match mode.as_str() {
        "encode" => encode_component(&read_text(values)?),
        "decode" => decode_component(&read_text(values)?),
        "query" => {
            if values.is_empty() {
                return Err("query mode needs key=value pairs".to_string());
            }
            format_query(values)
        }
        _ => Err(format!(
            "unknown mode '{mode}'; use encode, decode, or query"
        )),
    }
}

fn read_text(values: &[String]) -> Result<String, String> {
    if !values.is_empty() {
        return Ok(values.join(" "));
    }

    let mut input = String::new();
    io::stdin()
        .read_to_string(&mut input)
        .map_err(|error| format!("failed to read standard input: {error}"))?;
    Ok(input)
}

fn encode_component(input: &str) -> Result<String, String> {
    let mut encoded = String::new();

    for byte in input.bytes() {
        if byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'.' | b'_' | b'~') {
            encoded.push(char::from(byte));
        } else {
            encoded.push_str(&format!("%{byte:02X}"));
        }
    }

    Ok(encoded)
}

fn decode_component(input: &str) -> Result<String, String> {
    let bytes = input.as_bytes();
    let mut decoded = Vec::with_capacity(bytes.len());
    let mut index = 0;

    while index < bytes.len() {
        if bytes[index] != b'%' {
            decoded.push(bytes[index]);
            index += 1;
            continue;
        }

        let high = *bytes
            .get(index + 1)
            .ok_or_else(|| format!("incomplete percent escape at position {index}"))?;
        let low = *bytes
            .get(index + 2)
            .ok_or_else(|| format!("incomplete percent escape at position {index}"))?;
        let high =
            hex_value(high).ok_or_else(|| format!("invalid percent escape at position {index}"))?;
        let low =
            hex_value(low).ok_or_else(|| format!("invalid percent escape at position {index}"))?;
        decoded.push(high << 4 | low);
        index += 3;
    }

    String::from_utf8(decoded).map_err(|_| "decoded bytes are not valid UTF-8 text".to_string())
}

fn hex_value(byte: u8) -> Option<u8> {
    match byte {
        b'0'..=b'9' => Some(byte - b'0'),
        b'a'..=b'f' => Some(byte - b'a' + 10),
        b'A'..=b'F' => Some(byte - b'A' + 10),
        _ => None,
    }
}

fn format_query(pairs: &[String]) -> Result<String, String> {
    pairs
        .iter()
        .map(|pair| {
            let (key, value) = pair
                .split_once('=')
                .ok_or_else(|| format!("query pair '{pair}' must use key=value"))?;
            Ok(format!(
                "{}={}",
                encode_component(key)?,
                encode_component(value)?
            ))
        })
        .collect::<Result<Vec<_>, String>>()
        .map(|parts| parts.join("&"))
}

#[cfg(test)]
mod tests {
    use super::{decode_component, encode_component, format_query};

    #[test]
    fn encodes_spaces_symbols_and_utf8_bytes() {
        assert_eq!(
            encode_component("hello world!"),
            Ok("hello%20world%21".to_string())
        );
        assert_eq!(encode_component("café"), Ok("caf%C3%A9".to_string()));
    }

    #[test]
    fn decodes_percent_escapes() {
        assert_eq!(
            decode_component("hello%20world%21"),
            Ok("hello world!".to_string())
        );
    }

    #[test]
    fn rejects_malformed_escapes() {
        assert!(decode_component("bad%2").is_err());
        assert!(decode_component("bad%XZ").is_err());
    }

    #[test]
    fn formats_query_pairs_as_encoded_components() {
        let pairs = ["q=rust lang".to_string(), "page=2".to_string()];
        assert_eq!(format_query(&pairs), Ok("q=rust%20lang&page=2".to_string()));
    }
}
