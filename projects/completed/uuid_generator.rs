// Solution flow:
// 1. Parse count, output-format, and optional namespace-name arguments from the CLI.
// 2. Generate version-4 UUID bytes from the operating system's random source for normal mode.
// 3. Or hash a namespace UUID and name into a deterministic version-5 UUID.
// 4. Set the UUID version and variant bits, format each value, and print one UUID per line.
use std::{
    convert::TryInto,
    env,
    fs::File,
    io::Read,
    sync::atomic::{AtomicU64, Ordering},
    time::{SystemTime, UNIX_EPOCH},
};

static FALLBACK_COUNTER: AtomicU64 = AtomicU64::new(0);

fn main() {
    match run(&env::args().skip(1).collect::<Vec<_>>()) {
        Ok(output) => print!("{output}"),
        Err(error) => eprintln!("Error: {error}"),
    }
}

fn run(arguments: &[String]) -> Result<String, String> {
    let Options {
        count,
        format,
        namespace,
    } = parse_options(arguments)?;
    let uuids = match namespace {
        Some((namespace, name)) => vec![namespace_uuid(namespace, &name)],
        None => (0..count).map(|_| random_uuid()).collect(),
    };

    Ok(uuids
        .iter()
        .map(|uuid| format_uuid(*uuid, format))
        .collect::<Vec<_>>()
        .join("\n")
        + "\n")
}

#[derive(Clone, Copy)]
enum OutputFormat {
    Plain,
    Uppercase,
    Urn,
}

struct Options {
    count: usize,
    format: OutputFormat,
    namespace: Option<([u8; 16], String)>,
}

fn parse_options(arguments: &[String]) -> Result<Options, String> {
    let mut count = 1;
    let mut format = OutputFormat::Plain;
    let mut namespace = None;
    let mut index = 0;

    while index < arguments.len() {
        match arguments[index].as_str() {
            "--count" => {
                index += 1;
                let value = arguments
                    .get(index)
                    .ok_or_else(|| "--count needs a positive number".to_string())?;
                count = value
                    .parse::<usize>()
                    .map_err(|_| "--count must be a positive number".to_string())?;
                if count == 0 {
                    return Err("--count must be greater than zero".to_string());
                }
            }
            "--format" => {
                index += 1;
                format = match arguments
                    .get(index)
                    .ok_or_else(|| "--format needs plain, uppercase, or urn".to_string())?
                    .as_str()
                {
                    "plain" => OutputFormat::Plain,
                    "uppercase" => OutputFormat::Uppercase,
                    "urn" => OutputFormat::Urn,
                    value => return Err(format!("unknown output format '{value}'")),
                };
            }
            "--namespace" => {
                let namespace_text = arguments
                    .get(index + 1)
                    .ok_or_else(|| "--namespace needs a UUID and a name".to_string())?;
                let name = arguments
                    .get(index + 2)
                    .ok_or_else(|| "--namespace needs a UUID and a name".to_string())?;
                namespace = Some((parse_uuid(namespace_text)?, name.clone()));
                index += 2;
            }
            value => return Err(format!("unknown option '{value}'")),
        }
        index += 1;
    }

    if namespace.is_some() && count != 1 {
        return Err("--count cannot be combined with --namespace".to_string());
    }

    Ok(Options {
        count,
        format,
        namespace,
    })
}

fn random_uuid() -> [u8; 16] {
    let mut bytes = [0; 16];
    if File::open("/dev/urandom")
        .and_then(|mut file| file.read_exact(&mut bytes))
        .is_err()
    {
        bytes = fallback_random_bytes();
    }
    set_uuid_bits(&mut bytes, 4);
    bytes
}

fn fallback_random_bytes() -> [u8; 16] {
    let time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos() as u64;
    let counter = FALLBACK_COUNTER.fetch_add(1, Ordering::Relaxed);
    let first = splitmix64(time ^ counter ^ u64::from(std::process::id()));
    let second = splitmix64(first ^ counter.rotate_left(17));
    let mut bytes = [0; 16];
    bytes[..8].copy_from_slice(&first.to_be_bytes());
    bytes[8..].copy_from_slice(&second.to_be_bytes());
    bytes
}

fn splitmix64(mut value: u64) -> u64 {
    value = value.wrapping_add(0x9E37_79B9_7F4A_7C15);
    value = (value ^ (value >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
    value = (value ^ (value >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
    value ^ (value >> 31)
}

fn namespace_uuid(namespace: [u8; 16], name: &str) -> [u8; 16] {
    let mut input = namespace.to_vec();
    input.extend_from_slice(name.as_bytes());
    let hash = sha1(&input);
    let mut uuid = [0; 16];
    uuid.copy_from_slice(&hash[..16]);
    set_uuid_bits(&mut uuid, 5);
    uuid
}

fn set_uuid_bits(uuid: &mut [u8; 16], version: u8) {
    uuid[6] = (uuid[6] & 0x0F) | (version << 4);
    uuid[8] = (uuid[8] & 0x3F) | 0x80;
}

fn parse_uuid(input: &str) -> Result<[u8; 16], String> {
    let compact = input
        .strip_prefix("urn:uuid:")
        .unwrap_or(input)
        .replace('-', "");
    if compact.len() != 32 {
        return Err(format!("'{input}' is not a UUID"));
    }

    let mut uuid = [0; 16];
    for (index, slot) in uuid.iter_mut().enumerate() {
        *slot = u8::from_str_radix(&compact[index * 2..index * 2 + 2], 16)
            .map_err(|_| format!("'{input}' is not a UUID"))?;
    }
    Ok(uuid)
}

fn format_uuid(uuid: [u8; 16], format: OutputFormat) -> String {
    let value = format!(
        "{:02x}{:02x}{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}",
        uuid[0], uuid[1], uuid[2], uuid[3], uuid[4], uuid[5], uuid[6], uuid[7], uuid[8], uuid[9], uuid[10], uuid[11], uuid[12], uuid[13], uuid[14], uuid[15]
    );
    match format {
        OutputFormat::Plain => value,
        OutputFormat::Uppercase => value.to_ascii_uppercase(),
        OutputFormat::Urn => format!("urn:uuid:{value}"),
    }
}

fn sha1(input: &[u8]) -> [u8; 20] {
    let mut message = input.to_vec();
    let bit_length = (message.len() as u64).wrapping_mul(8);
    message.push(0x80);
    while message.len() % 64 != 56 {
        message.push(0);
    }
    message.extend_from_slice(&bit_length.to_be_bytes());

    let mut hash = [
        0x6745_2301u32,
        0xEFCD_AB89,
        0x98BA_DCFE,
        0x1032_5476,
        0xC3D2_E1F0,
    ];

    for chunk in message.chunks_exact(64) {
        let mut words = [0u32; 80];
        for (index, word) in words[..16].iter_mut().enumerate() {
            *word = u32::from_be_bytes(chunk[index * 4..index * 4 + 4].try_into().unwrap());
        }
        for index in 16..80 {
            words[index] =
                (words[index - 3] ^ words[index - 8] ^ words[index - 14] ^ words[index - 16])
                    .rotate_left(1);
        }

        let (mut a, mut b, mut c, mut d, mut e) = (hash[0], hash[1], hash[2], hash[3], hash[4]);
        for (index, word) in words.iter().enumerate() {
            let (function, constant) = match index {
                0..=19 => ((b & c) | ((!b) & d), 0x5A82_7999),
                20..=39 => (b ^ c ^ d, 0x6ED9_EBA1),
                40..=59 => ((b & c) | (b & d) | (c & d), 0x8F1B_BCDC),
                _ => (b ^ c ^ d, 0xCA62_C1D6),
            };
            let temporary = a
                .rotate_left(5)
                .wrapping_add(function)
                .wrapping_add(e)
                .wrapping_add(constant)
                .wrapping_add(*word);
            e = d;
            d = c;
            c = b.rotate_left(30);
            b = a;
            a = temporary;
        }

        hash[0] = hash[0].wrapping_add(a);
        hash[1] = hash[1].wrapping_add(b);
        hash[2] = hash[2].wrapping_add(c);
        hash[3] = hash[3].wrapping_add(d);
        hash[4] = hash[4].wrapping_add(e);
    }

    let mut output = [0; 20];
    for (index, value) in hash.iter().enumerate() {
        output[index * 4..index * 4 + 4].copy_from_slice(&value.to_be_bytes());
    }
    output
}

#[cfg(test)]
mod tests {
    use super::{format_uuid, namespace_uuid, parse_uuid, random_uuid, OutputFormat};

    #[test]
    fn generated_uuid_has_version_four_and_rfc_variant_bits() {
        let uuid = random_uuid();
        assert_eq!(uuid[6] >> 4, 4);
        assert_eq!(uuid[8] >> 6, 2);
    }

    #[test]
    fn formats_and_parses_uuid_shapes() {
        let uuid = parse_uuid("123e4567-e89b-12d3-a456-426614174000").unwrap();
        assert_eq!(
            format_uuid(uuid, OutputFormat::Plain),
            "123e4567-e89b-12d3-a456-426614174000"
        );
        assert_eq!(
            format_uuid(uuid, OutputFormat::Uppercase),
            "123E4567-E89B-12D3-A456-426614174000"
        );
        assert_eq!(
            format_uuid(uuid, OutputFormat::Urn),
            "urn:uuid:123e4567-e89b-12d3-a456-426614174000"
        );
    }

    #[test]
    fn generates_rfc_namespace_uuid() {
        let dns = parse_uuid("6ba7b810-9dad-11d1-80b4-00c04fd430c8").unwrap();
        let uuid = namespace_uuid(dns, "www.widgets.com");
        assert_eq!(
            format_uuid(uuid, OutputFormat::Plain),
            "21f7f8de-8051-5b89-8680-0195ef798b6a"
        );
    }

    #[test]
    fn rejects_invalid_uuid_input() {
        assert!(parse_uuid("not-a-uuid").is_err());
    }
}
