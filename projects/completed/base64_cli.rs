// Solution flow:
// 1. Parse `encode` or `decode` mode plus optional URL-safe and file-input flags.
// 2. Read bytes from text, a file, or standard input so binary data stays intact.
// 3. Convert bytes to Base64 groups of four characters, or validate and decode those groups.
// 4. Write the result to standard output and report malformed Base64 as an error.
use std::{
    env, fs,
    io::{self, Read, Write},
};

const STANDARD_ALPHABET: &[u8; 64] =
    b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
const URL_SAFE_ALPHABET: &[u8; 64] =
    b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-_";

fn main() {
    match run(&env::args().skip(1).collect::<Vec<_>>()) {
        Ok(()) => {}
        Err(error) => eprintln!("Error: {error}"),
    }
}

fn run(arguments: &[String]) -> Result<(), String> {
    let options = parse_options(arguments)?;
    let input = read_input(&options.input)?;
    let output = match options.mode {
        Mode::Encode => encode_base64(&input, options.url_safe).into_bytes(),
        Mode::Decode => decode_base64(&input, options.url_safe)?,
    };

    let mut stdout = io::stdout().lock();
    stdout
        .write_all(&output)
        .map_err(|error| format!("failed to write output: {error}"))?;
    if matches!(options.mode, Mode::Encode) {
        stdout
            .write_all(b"\n")
            .map_err(|error| format!("failed to write output: {error}"))?;
    }
    Ok(())
}

#[derive(Clone, Copy)]
enum Mode {
    Encode,
    Decode,
}

enum Input {
    Text(String),
    File(String),
    Stdin,
}

struct Options {
    mode: Mode,
    url_safe: bool,
    input: Input,
}

fn parse_options(arguments: &[String]) -> Result<Options, String> {
    let (mode, remaining) = arguments.split_first().ok_or_else(|| {
        "usage: base64_cli <encode|decode> [--url-safe] [--file PATH] [text]".to_string()
    })?;
    let mode = match mode.as_str() {
        "encode" => Mode::Encode,
        "decode" => Mode::Decode,
        _ => return Err(format!("unknown mode '{mode}'; use encode or decode")),
    };

    let mut url_safe = false;
    let mut file = None;
    let mut text = Vec::new();
    let mut index = 0;

    while index < remaining.len() {
        match remaining[index].as_str() {
            "--url-safe" => url_safe = true,
            "--file" => {
                index += 1;
                let path = remaining
                    .get(index)
                    .ok_or_else(|| "--file needs a path".to_string())?;
                file = Some(path.clone());
            }
            value if value.starts_with("--") => return Err(format!("unknown option '{value}'")),
            value => text.push(value.to_string()),
        }
        index += 1;
    }

    if file.is_some() && !text.is_empty() {
        return Err("use either --file or text input, not both".to_string());
    }

    let input = match file {
        Some(path) => Input::File(path),
        None if !text.is_empty() => Input::Text(text.join(" ")),
        None => Input::Stdin,
    };

    Ok(Options {
        mode,
        url_safe,
        input,
    })
}

fn read_input(input: &Input) -> Result<Vec<u8>, String> {
    match input {
        Input::Text(text) => Ok(text.as_bytes().to_vec()),
        Input::File(path) => {
            fs::read(path).map_err(|error| format!("failed to read '{path}': {error}"))
        }
        Input::Stdin => {
            let mut bytes = Vec::new();
            io::stdin()
                .read_to_end(&mut bytes)
                .map_err(|error| format!("failed to read standard input: {error}"))?;
            Ok(bytes)
        }
    }
}

fn encode_base64(input: &[u8], url_safe: bool) -> String {
    let alphabet = if url_safe {
        URL_SAFE_ALPHABET
    } else {
        STANDARD_ALPHABET
    };
    let mut encoded = String::with_capacity(input.len().div_ceil(3) * 4);

    for chunk in input.chunks(3) {
        let first = chunk[0];
        let second = *chunk.get(1).unwrap_or(&0);
        let third = *chunk.get(2).unwrap_or(&0);

        encoded.push(char::from(alphabet[usize::from(first >> 2)]));
        encoded.push(char::from(
            alphabet[usize::from((first & 0b0000_0011) << 4 | second >> 4)],
        ));
        encoded.push(if chunk.len() > 1 {
            char::from(alphabet[usize::from((second & 0b0000_1111) << 2 | third >> 6)])
        } else {
            '='
        });
        encoded.push(if chunk.len() > 2 {
            char::from(alphabet[usize::from(third & 0b0011_1111)])
        } else {
            '='
        });
    }

    encoded
}

fn decode_base64(input: &[u8], url_safe: bool) -> Result<Vec<u8>, String> {
    let mut characters = input
        .iter()
        .copied()
        .filter(|byte| !byte.is_ascii_whitespace())
        .collect::<Vec<_>>();

    if characters.len() % 4 == 1 {
        return Err("Base64 input has an invalid length".to_string());
    }
    while characters.len() % 4 != 0 {
        characters.push(b'=');
    }

    let mut decoded = Vec::with_capacity(characters.len() / 4 * 3);
    for (chunk_index, chunk) in characters.chunks_exact(4).enumerate() {
        let is_last_chunk = chunk_index + 1 == characters.len() / 4;
        let first = decode_value(chunk[0], url_safe)?;
        let second = decode_value(chunk[1], url_safe)?;
        let third = decode_optional_value(chunk[2], url_safe)?;
        let fourth = decode_optional_value(chunk[3], url_safe)?;

        if !is_last_chunk && (third.is_none() || fourth.is_none()) {
            return Err("padding is only valid in the final Base64 group".to_string());
        }
        if third.is_none() && fourth.is_some() {
            return Err("invalid Base64 padding".to_string());
        }

        decoded.push(first << 2 | second >> 4);
        if let Some(third) = third {
            decoded.push(second << 4 | third >> 2);
            if let Some(fourth) = fourth {
                decoded.push(third << 6 | fourth);
            }
        }
    }

    Ok(decoded)
}

fn decode_optional_value(byte: u8, url_safe: bool) -> Result<Option<u8>, String> {
    if byte == b'=' {
        Ok(None)
    } else {
        decode_value(byte, url_safe).map(Some)
    }
}

fn decode_value(byte: u8, url_safe: bool) -> Result<u8, String> {
    let value = match byte {
        b'A'..=b'Z' => byte - b'A',
        b'a'..=b'z' => byte - b'a' + 26,
        b'0'..=b'9' => byte - b'0' + 52,
        b'+' if !url_safe => 62,
        b'/' if !url_safe => 63,
        b'-' if url_safe => 62,
        b'_' if url_safe => 63,
        _ => return Err(format!("'{0}' is not valid Base64 input", char::from(byte))),
    };
    Ok(value)
}

#[cfg(test)]
mod tests {
    use super::{decode_base64, encode_base64};

    #[test]
    fn encodes_known_text() {
        assert_eq!(encode_base64(b"Hello", false), "SGVsbG8=");
    }

    #[test]
    fn round_trips_binary_data() {
        let bytes = [0, 1, 2, 250, 251, 252, 255];
        let encoded = encode_base64(&bytes, false);
        assert_eq!(decode_base64(encoded.as_bytes(), false), Ok(bytes.to_vec()));
    }

    #[test]
    fn supports_url_safe_alphabet_and_unpadded_input() {
        assert_eq!(encode_base64(&[251, 255], true), "-_8=");
        assert_eq!(decode_base64(b"-_8", true), Ok(vec![251, 255]));
    }

    #[test]
    fn reports_invalid_base64() {
        assert!(decode_base64(b"abc$", false).is_err());
        assert!(decode_base64(b"a", false).is_err());
        assert!(decode_base64(b"ab=c", false).is_err());
    }
}
