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
    width: usize,
    color: bool,
    input: Option<String>,
}

fn run(arguments: &[String]) -> Result<String, String> {
    let options = parse_options(arguments)?;
    let bytes = match options.input.as_deref() {
        Some(path) => fs::read(path).map_err(|e| format!("failed to read '{path}': {e}"))?,
        None => {
            let mut bytes = Vec::new();
            io::stdin()
                .read_to_end(&mut bytes)
                .map_err(|e| format!("failed to read standard input: {e}"))?;
            bytes
        }
    };
    Ok(render_hex(&bytes, options.width, options.color))
}

fn parse_options(arguments: &[String]) -> Result<Options, String> {
    let mut options = Options {
        width: 16,
        color: false,
        input: None,
    };
    let mut index = 0;
    while index < arguments.len() {
        match arguments[index].as_str() {
            "--bytes-per-row" => {
                index += 1;
                options.width = arguments
                    .get(index)
                    .ok_or_else(|| "--bytes-per-row needs a number".to_string())?
                    .parse()
                    .map_err(|_| "--bytes-per-row must be a number".to_string())?;
                if options.width == 0 || options.width > 64 {
                    return Err("--bytes-per-row must be between 1 and 64".to_string());
                }
            }
            "--color" => options.color = true,
            value if value.starts_with('-') => return Err(format!("unknown option '{value}'")),
            path => {
                if options.input.replace(path.to_string()).is_some() {
                    return Err("only one input file is supported".to_string());
                }
            }
        }
        index += 1;
    }
    Ok(options)
}

fn render_hex(bytes: &[u8], width: usize, color: bool) -> String {
    bytes
        .chunks(width)
        .enumerate()
        .map(|(row, chunk)| {
            let hex = (0..width)
                .map(|index| match chunk.get(index) {
                    Some(byte) if color => format!(
                        "\x1b[{}m{byte:02x}\x1b[0m",
                        if byte.is_ascii_graphic() { 32 } else { 33 }
                    ),
                    Some(byte) => format!("{byte:02x}"),
                    None => "  ".to_string(),
                })
                .collect::<Vec<_>>()
                .join(" ");
            let ascii = chunk
                .iter()
                .map(|byte| {
                    if byte.is_ascii_graphic() || *byte == b' ' {
                        *byte as char
                    } else {
                        '.'
                    }
                })
                .collect::<String>();
            format!(
                "{offset:08x}  {hex:<width$}  |{ascii}|\n",
                offset = row * width,
                width = width * 3 - 1
            )
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::{parse_options, render_hex};

    #[test]
    fn renders_offsets_hex_and_ascii() {
        assert_eq!(
            render_hex(b"A\nB", 2, false),
            "00000000  41 0a  |A.|\n00000002  42     |B|\n"
        );
    }

    #[test]
    fn handles_short_final_rows_and_width_options() {
        let options = parse_options(&["--bytes-per-row".to_string(), "4".to_string()]).unwrap();
        assert_eq!(options.width, 4);
        assert_eq!(
            render_hex(b"abc", 4, false),
            "00000000  61 62 63     |abc|\n"
        );
    }

    #[test]
    fn rejects_invalid_row_widths() {
        assert!(parse_options(&["--bytes-per-row".to_string(), "0".to_string()]).is_err());
        assert!(parse_options(&["--bytes-per-row".to_string(), "65".to_string()]).is_err());
    }
}
