// Solution flow:
// 1. Read one or more hexadecimal, RGB, or named colors from the command line.
// 2. Parse each input into one RGB value so validation is separate from terminal rendering.
// 3. Render an ANSI true-color swatch with matching hex and RGB summaries.
// 4. Support a named-palette view and xterm ANSI-256 lookup as useful preview shortcuts.
use std::env;

fn main() {
    match run(&env::args().skip(1).collect::<Vec<_>>()) {
        Ok(output) => print!("{output}"),
        Err(error) => eprintln!("Error: {error}"),
    }
}

fn run(arguments: &[String]) -> Result<String, String> {
    match arguments {
        [flag] if flag == "--palette" => Ok(render_palette()),
        [flag, code] if flag == "--ansi" => {
            let code = code
                .parse::<u8>()
                .map_err(|_| "--ansi needs a value from 0 to 255".to_string())?;
            Ok(render_preview(ansi_256_color(code)))
        }
        [] => Err(
            "usage: color_preview <#RRGGBB|r,g,b|name>... | --palette | --ansi CODE".to_string(),
        ),
        values => values
            .iter()
            .map(|value| parse_color(value).map(render_preview))
            .collect::<Result<Vec<_>, _>>()
            .map(|previews| previews.join("\n") + "\n"),
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

const NAMED_PALETTE: &[(&str, Color)] = &[
    (
        "red",
        Color {
            red: 220,
            green: 50,
            blue: 47,
        },
    ),
    (
        "green",
        Color {
            red: 133,
            green: 153,
            blue: 0,
        },
    ),
    (
        "blue",
        Color {
            red: 38,
            green: 139,
            blue: 210,
        },
    ),
    (
        "yellow",
        Color {
            red: 181,
            green: 137,
            blue: 0,
        },
    ),
    (
        "purple",
        Color {
            red: 108,
            green: 113,
            blue: 196,
        },
    ),
];

fn parse_color(input: &str) -> Result<Color, String> {
    let input = input.trim();
    if let Some(color) = NAMED_PALETTE
        .iter()
        .find_map(|(name, color)| name.eq_ignore_ascii_case(input).then_some(*color))
    {
        return Ok(color);
    }

    if let Some(hexadecimal) = input.strip_prefix('#') {
        return parse_hex_color(hexadecimal);
    }
    if let Some(rgb) = input
        .strip_prefix("rgb(")
        .and_then(|value| value.strip_suffix(')'))
    {
        return parse_rgb_color(rgb);
    }

    parse_rgb_color(input)
}

fn parse_hex_color(hexadecimal: &str) -> Result<Color, String> {
    let expanded = match hexadecimal.len() {
        3 => hexadecimal
            .chars()
            .flat_map(|character| [character, character])
            .collect::<String>(),
        6 => hexadecimal.to_string(),
        _ => {
            return Err(format!(
                "'{hexadecimal}' must contain 3 or 6 hexadecimal digits"
            ))
        }
    };

    let component = |start| {
        u8::from_str_radix(&expanded[start..start + 2], 16)
            .map_err(|_| format!("'{hexadecimal}' is not a valid hexadecimal color"))
    };

    Ok(Color {
        red: component(0)?,
        green: component(2)?,
        blue: component(4)?,
    })
}

fn parse_rgb_color(input: &str) -> Result<Color, String> {
    let components = input.split(',').map(str::trim).collect::<Vec<_>>();
    if components.len() != 3 {
        return Err(format!("'{input}' must use r,g,b values"));
    }
    let parse_component = |component: &str| {
        component
            .parse::<u8>()
            .map_err(|_| format!("'{component}' is not an RGB value from 0 to 255"))
    };

    Ok(Color {
        red: parse_component(components[0])?,
        green: parse_component(components[1])?,
        blue: parse_component(components[2])?,
    })
}

fn render_preview(color: Color) -> String {
    format!(
        "\x1b[48;2;{};{};{}m    \x1b[0m #{:02X}{:02X}{:02X} rgb({}, {}, {})",
        color.red,
        color.green,
        color.blue,
        color.red,
        color.green,
        color.blue,
        color.red,
        color.green,
        color.blue
    )
}

fn render_palette() -> String {
    NAMED_PALETTE
        .iter()
        .map(|(name, color)| format!("{name:<6} {}", render_preview(*color)))
        .collect::<Vec<_>>()
        .join("\n")
        + "\n"
}

fn ansi_256_color(code: u8) -> Color {
    const BASIC: [Color; 16] = [
        Color {
            red: 0,
            green: 0,
            blue: 0,
        },
        Color {
            red: 205,
            green: 49,
            blue: 49,
        },
        Color {
            red: 13,
            green: 188,
            blue: 121,
        },
        Color {
            red: 229,
            green: 229,
            blue: 16,
        },
        Color {
            red: 36,
            green: 114,
            blue: 200,
        },
        Color {
            red: 188,
            green: 63,
            blue: 188,
        },
        Color {
            red: 17,
            green: 168,
            blue: 205,
        },
        Color {
            red: 229,
            green: 229,
            blue: 229,
        },
        Color {
            red: 102,
            green: 102,
            blue: 102,
        },
        Color {
            red: 241,
            green: 76,
            blue: 76,
        },
        Color {
            red: 35,
            green: 209,
            blue: 139,
        },
        Color {
            red: 245,
            green: 245,
            blue: 67,
        },
        Color {
            red: 59,
            green: 142,
            blue: 234,
        },
        Color {
            red: 214,
            green: 112,
            blue: 214,
        },
        Color {
            red: 41,
            green: 184,
            blue: 219,
        },
        Color {
            red: 255,
            green: 255,
            blue: 255,
        },
    ];

    match code {
        0..=15 => BASIC[usize::from(code)],
        16..=231 => {
            let value = code - 16;
            let steps = [0, 95, 135, 175, 215, 255];
            Color {
                red: steps[usize::from(value / 36)],
                green: steps[usize::from(value / 6 % 6)],
                blue: steps[usize::from(value % 6)],
            }
        }
        232..=255 => {
            let gray = 8 + (code - 232) * 10;
            Color {
                red: gray,
                green: gray,
                blue: gray,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{ansi_256_color, parse_color, render_preview, Color};

    #[test]
    fn parses_hex_and_rgb_inputs() {
        assert_eq!(
            parse_color("#0f8"),
            Ok(Color {
                red: 0,
                green: 255,
                blue: 136
            })
        );
        assert_eq!(
            parse_color("rgb(12, 34, 56)"),
            Ok(Color {
                red: 12,
                green: 34,
                blue: 56
            })
        );
    }

    #[test]
    fn rejects_malformed_colors() {
        assert!(parse_color("#12345").is_err());
        assert!(parse_color("300,0,0").is_err());
        assert!(parse_color("red,blue").is_err());
    }

    #[test]
    fn renders_a_true_color_swatch_and_summary() {
        let color = Color {
            red: 12,
            green: 34,
            blue: 56,
        };
        assert_eq!(
            render_preview(color),
            "\x1b[48;2;12;34;56m    \x1b[0m #0C2238 rgb(12, 34, 56)"
        );
    }

    #[test]
    fn maps_ansi_256_color_cube_and_grayscale() {
        assert_eq!(
            ansi_256_color(16),
            Color {
                red: 0,
                green: 0,
                blue: 0
            }
        );
        assert_eq!(
            ansi_256_color(21),
            Color {
                red: 0,
                green: 0,
                blue: 255
            }
        );
        assert_eq!(
            ansi_256_color(232),
            Color {
                red: 8,
                green: 8,
                blue: 8
            }
        );
    }
}
