use aes_gcm::{Aes256Gcm, KeyInit, Nonce, aead::Aead};
use base64::{Engine, engine::general_purpose::STANDARD};
use sha2::{Digest, Sha256};
use std::{collections::BTreeMap, env, fs, path::Path};

fn main() {
    match run(&env::args().skip(1).collect::<Vec<_>>()) {
        Ok(output) => print!("{output}"),
        Err(error) => eprintln!("Error: {error}"),
    }
}

fn run(args: &[String]) -> Result<String, String> {
    match args {
        [file, key, command] if command == "list" => {
            let store = load(file, key)?;
            Ok(store.keys().cloned().collect::<Vec<_>>().join("\n") + "\n")
        }
        [file, key, command, name, secret] if command == "add" => {
            let mut store = load(file, key)?;
            store.insert(name.clone(), secret.clone());
            save(file, key, &store)?;
            Ok(format!("stored {name}\n"))
        }
        [file, key, command, name] if command == "get" => {
            let store = load(file, key)?;
            store
                .get(name)
                .cloned()
                .ok_or_else(|| format!("secret '{name}' not found"))
        }
        [file, key, command, name] if command == "delete" => {
            let mut store = load(file, key)?;
            if store.remove(name).is_none() {
                return Err(format!("secret '{name}' not found"));
            }
            save(file, key, &store)?;
            Ok(format!("deleted {name}\n"))
        }
        [file, _key, command] if command == "generate" => {
            Ok(format!("{}\n", generate_password(24)))
        }
        _ => Err(
            "usage: password_store_cli FILE KEY list|add NAME SECRET|get NAME|delete NAME|generate"
                .to_string(),
        ),
    }
}

fn derive_key(key: &str) -> [u8; 32] {
    Sha256::digest(key.as_bytes()).into()
}

fn load(path: &str, key: &str) -> Result<BTreeMap<String, String>, String> {
    if !Path::new(path).exists() {
        return Ok(BTreeMap::new());
    }
    let encoded =
        fs::read_to_string(path).map_err(|error| format!("failed to read store: {error}"))?;
    if encoded.trim().is_empty() {
        return Ok(BTreeMap::new());
    }
    let bytes = STANDARD
        .decode(encoded.trim())
        .map_err(|_| "store is not valid base64".to_string())?;
    if bytes.len() < 12 {
        return Err("store is truncated".to_string());
    }
    let cipher =
        Aes256Gcm::new_from_slice(&derive_key(key)).map_err(|_| "invalid key".to_string())?;
    let plaintext = cipher
        .decrypt(Nonce::from_slice(&bytes[..12]), &bytes[12..])
        .map_err(|_| "wrong key or corrupt store".to_string())?;
    let mut store = BTreeMap::new();
    for line in String::from_utf8(plaintext)
        .map_err(|_| "store contains invalid text".to_string())?
        .lines()
    {
        let (name, secret) = line
            .split_once('\t')
            .ok_or_else(|| "store record is malformed".to_string())?;
        store.insert(name.to_string(), secret.to_string());
    }
    Ok(store)
}

fn save(path: &str, key: &str, store: &BTreeMap<String, String>) -> Result<(), String> {
    let plaintext = store
        .iter()
        .map(|(name, secret)| format!("{name}\t{secret}\n"))
        .collect::<String>();
    let nonce: [u8; 12] = rand::random();
    let cipher =
        Aes256Gcm::new_from_slice(&derive_key(key)).map_err(|_| "invalid key".to_string())?;
    let ciphertext = cipher
        .encrypt(Nonce::from_slice(&nonce), plaintext.as_bytes())
        .map_err(|_| "encryption failed".to_string())?;
    let mut output = nonce.to_vec();
    output.extend(ciphertext);
    fs::write(path, STANDARD.encode(output))
        .map_err(|error| format!("failed to write store: {error}"))
}

fn generate_password(length: usize) -> String {
    const ALPHABET: &[u8] = b"ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz23456789!@#$%^&*";
    (0..length)
        .map(|_| ALPHABET[(rand::random::<u64>() as usize) % ALPHABET.len()] as char)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::{generate_password, load, save};

    #[test]
    fn encrypts_round_trip_and_rejects_bad_key() {
        let directory = tempfile::tempdir().unwrap();
        let path = directory.path().join("store");
        let mut store = std::collections::BTreeMap::new();
        store.insert("email".to_string(), "secret".to_string());
        save(path.to_str().unwrap(), "correct", &store).unwrap();
        assert_eq!(load(path.to_str().unwrap(), "correct").unwrap(), store);
        assert!(load(path.to_str().unwrap(), "wrong").is_err());
    }

    #[test]
    fn generated_password_has_requested_length() {
        assert_eq!(generate_password(32).len(), 32);
    }
}
