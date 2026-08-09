use data_encoding::BASE32;
use hmac::{Hmac, Mac};
use sha1::Sha1;
use std::{collections::BTreeMap, env, fs};

type HmacSha1 = Hmac<Sha1>;

fn main() {
    match run(&env::args().skip(1).collect::<Vec<_>>()) {
        Ok(output) => println!("{output}"),
        Err(error) => eprintln!("Error: {error}"),
    }
}

fn run(args: &[String]) -> Result<String, String> {
    match args {
        [command, secret] if command == "code" => Ok(format!("{:06}", generate_code(secret, 30, current_time())?)),
        [command, secret, timestamp] if command == "code" => {
            let time = timestamp.parse().map_err(|_| "timestamp must be an integer".to_string())?;
            Ok(format!("{:06}", generate_code(secret, 30, time)?))
        }
        [command, file, name, secret] if command == "add" => {
            let mut accounts = load_accounts(file)?;
            accounts.insert(name.clone(), secret.clone());
            save_accounts(file, &accounts)?;
            Ok(format!("stored {name}"))
        }
        [command, file, _name] if command == "list" => {
            let accounts = load_accounts(file)?;
            Ok(accounts.keys().cloned().collect::<Vec<_>>().join("\n"))
        }
        [command, file, name, timestamp] if command == "account" => {
            let accounts = load_accounts(file)?;
            let secret = accounts.get(name).ok_or_else(|| format!("account '{name}' not found"))?;
            let time = timestamp.parse().map_err(|_| "timestamp must be an integer".to_string())?;
            Ok(format!("{:06}", generate_code(secret, 30, time)?))
        }
        [command, uri] if command == "import" => {
            let (label, _, period) = parse_otpauth_uri(uri)?;
            Ok(format!("imported {label} (period {period}s)"))
        }
        _ => Err("usage: totp_manager code SECRET [UNIX_TIME] | add FILE NAME SECRET | list FILE | account FILE NAME UNIX_TIME | import OTPAUTH_URI".to_string()),
    }
}

fn parse_otpauth_uri(uri: &str) -> Result<(String, String, u64), String> {
    let (scheme, rest) = uri
        .split_once("//")
        .ok_or_else(|| "invalid otpauth URI".to_string())?;
    if scheme != "otpauth:" || !rest.starts_with("totp/") {
        return Err("URI must use otpauth://totp/".to_string());
    }
    let (label, query) = rest[5..]
        .split_once('?')
        .ok_or_else(|| "otpauth URI needs query parameters".to_string())?;
    let secret = query
        .split('&')
        .find_map(|part| part.strip_prefix("secret="))
        .ok_or_else(|| "otpauth URI needs a secret".to_string())?
        .to_string();
    generate_code(&secret, 30, 0)?;
    let period = query
        .split('&')
        .find_map(|part| part.strip_prefix("period="))
        .map(|value| {
            value
                .parse()
                .map_err(|_| "period must be an integer".to_string())
        })
        .transpose()?
        .unwrap_or(30);
    Ok((label.to_string(), secret, period))
}

fn current_time() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}

fn generate_code(secret: &str, step: u64, timestamp: u64) -> Result<u32, String> {
    let normalized = secret.trim().replace(' ', "").to_uppercase();
    let key = BASE32
        .decode(normalized.as_bytes())
        .map_err(|_| "secret is not valid Base32".to_string())?;
    if key.is_empty() {
        return Err("secret cannot be empty".to_string());
    }
    let counter = timestamp / step;
    let mut mac = HmacSha1::new_from_slice(&key).map_err(|_| "invalid secret".to_string())?;
    mac.update(&counter.to_be_bytes());
    let digest = mac.finalize().into_bytes();
    let offset = usize::from(digest[19] & 0x0f);
    let binary = (u32::from(digest[offset]) & 0x7f) << 24
        | u32::from(digest[offset + 1]) << 16
        | u32::from(digest[offset + 2]) << 8
        | u32::from(digest[offset + 3]);
    Ok(binary % 1_000_000)
}

fn load_accounts(path: &str) -> Result<BTreeMap<String, String>, String> {
    let text =
        fs::read_to_string(path).map_err(|error| format!("failed to read accounts: {error}"))?;
    text.lines()
        .map(|line| {
            line.split_once('\t')
                .map(|(name, secret)| (name.to_string(), secret.to_string()))
                .ok_or_else(|| "account record is malformed".to_string())
        })
        .collect()
}

fn save_accounts(path: &str, accounts: &BTreeMap<String, String>) -> Result<(), String> {
    let text = accounts
        .iter()
        .map(|(name, secret)| format!("{name}\t{secret}\n"))
        .collect::<String>();
    fs::write(path, text).map_err(|error| format!("failed to write accounts: {error}"))
}

#[cfg(test)]
mod tests {
    use super::generate_code;

    #[test]
    fn matches_rfc_6238_sha1_vectors() {
        let secret = "GEZDGNBVGY3TQOJQGEZDGNBVGY3TQOJQ";
        assert_eq!(generate_code(secret, 30, 59).unwrap(), 287082);
        assert_eq!(generate_code(secret, 30, 1_111_111_109).unwrap(), 81804);
        assert_eq!(generate_code(secret, 30, 1_234_567_890).unwrap(), 5924);
    }

    #[test]
    fn rejects_invalid_secrets() {
        assert!(generate_code("not-valid!", 30, 0).is_err());
        assert!(super::parse_otpauth_uri("otpauth://totp/demo?secret=JBSWY3DPEHPK3PXP").is_ok());
    }
}
