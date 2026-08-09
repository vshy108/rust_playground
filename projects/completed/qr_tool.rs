use image::Luma;
use qrcode::{EcLevel, QrCode};
use std::env;

fn main() {
    match run(&env::args().skip(1).collect::<Vec<_>>()) {
        Ok(output) => print!("{output}"),
        Err(error) => eprintln!("Error: {error}"),
    }
}

fn run(args: &[String]) -> Result<String, String> {
    match args {
        [command, text] if command == "encode" => encode_terminal(text, EcLevel::M),
        [command, text, flag, level] if command == "encode" && flag == "--level" => {
            encode_terminal(text, parse_level(level)?)
        }
        [command, text, flag, path] if command == "encode" && flag == "--png" => {
            encode_png(text, path, EcLevel::M)
        }
        [command, path] if command == "decode" => decode_png(path),
        _ => Err(
            "usage: qr_tool encode TEXT [--level L|M|Q|H] | encode TEXT --png FILE | decode PNG"
                .to_string(),
        ),
    }
}

fn parse_level(value: &str) -> Result<EcLevel, String> {
    match value.to_uppercase().as_str() {
        "L" => Ok(EcLevel::L),
        "M" => Ok(EcLevel::M),
        "Q" => Ok(EcLevel::Q),
        "H" => Ok(EcLevel::H),
        _ => Err("error-correction level must be L, M, Q, or H".to_string()),
    }
}

fn encode_terminal(text: &str, level: EcLevel) -> Result<String, String> {
    let code = QrCode::with_error_correction_level(text.as_bytes(), level)
        .map_err(|error| format!("failed to encode QR: {error}"))?;
    Ok(code
        .render::<qrcode::render::unicode::Dense1x2>()
        .quiet_zone(true)
        .build()
        + "\n")
}

fn encode_png(text: &str, path: &str, level: EcLevel) -> Result<String, String> {
    let code = QrCode::with_error_correction_level(text.as_bytes(), level)
        .map_err(|error| format!("failed to encode QR: {error}"))?;
    code.render::<Luma<u8>>()
        .quiet_zone(true)
        .module_dimensions(8, 8)
        .build()
        .save(path)
        .map_err(|error| format!("failed to write '{path}': {error}"))?;
    Ok(format!("wrote {path}\n"))
}

fn decode_png(path: &str) -> Result<String, String> {
    let image = image::open(path)
        .map_err(|error| format!("failed to read '{path}': {error}"))?
        .to_luma8();
    let mut prepared = rqrr::PreparedImage::prepare(image);
    let grids = prepared.detect_grids();
    let (_, content) = grids
        .into_iter()
        .next()
        .ok_or_else(|| "no QR code found".to_string())?
        .decode()
        .map_err(|error| format!("failed to decode QR: {error:?}"))?;
    Ok(content)
}

#[cfg(test)]
mod tests {
    use super::{decode_png, encode_png, encode_terminal, parse_level};
    use std::fs;

    #[test]
    fn renders_deterministic_terminal_output() {
        let first = encode_terminal("hello", qrcode::EcLevel::M).unwrap();
        let second = encode_terminal("hello", qrcode::EcLevel::M).unwrap();
        assert_eq!(first, second);
        assert!(first.contains('█') || first.contains('▀'));
    }

    #[test]
    fn accepts_all_error_correction_levels() {
        for level in ["L", "M", "Q", "H"] {
            assert!(parse_level(level).is_ok());
        }
        assert!(parse_level("X").is_err());
    }

    #[test]
    fn exports_png_fixture() {
        let directory = tempfile::tempdir().unwrap();
        let path = directory.path().join("code.png");
        encode_png("fixture", path.to_str().unwrap(), qrcode::EcLevel::H).unwrap();
        assert!(fs::metadata(&path).unwrap().len() > 0);
        assert_eq!(decode_png(path.to_str().unwrap()).unwrap(), "fixture");
    }
}
