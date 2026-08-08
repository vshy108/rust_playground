use std::{
    env, fs,
    io::{self, Read},
    path::{Path, PathBuf},
};

fn main() {
    match run(&env::args().skip(1).collect::<Vec<_>>()) {
        Ok(output) => print!("{output}"),
        Err(error) => eprintln!("Error: {error}"),
    }
}

fn run(arguments: &[String]) -> Result<String, String> {
    if arguments.first().map(String::as_str) == Some("--verify") {
        let manifest = arguments
            .get(1)
            .ok_or_else(|| "usage: checksum_tool --verify MANIFEST".to_string())?;
        if arguments.len() > 2 {
            return Err("--verify accepts one manifest path".to_string());
        }
        return verify_manifest(manifest);
    }

    if arguments.first().map(String::as_str) == Some("--dir") {
        let root = arguments
            .get(1)
            .ok_or_else(|| "usage: checksum_tool --dir PATH [--extension EXT]".to_string())?;
        let extension = match arguments {
            [_, _] => None,
            [_, _, flag, value] if flag == "--extension" => Some(value.as_str()),
            _ => return Err("usage: checksum_tool --dir PATH [--extension EXT]".to_string()),
        };
        let mut paths = Vec::new();
        collect_files(Path::new(root), extension, &mut paths)?;
        paths.sort();
        return paths
            .iter()
            .map(|path| {
                let path = path.to_string_lossy();
                let bytes = read_input(&path)?;
                Ok(format_output(&sha256_hex(&bytes), &path))
            })
            .collect();
    }

    let paths = if arguments.is_empty() {
        vec!["-".to_string()]
    } else {
        arguments.to_vec()
    };

    paths
        .iter()
        .map(|path| {
            let bytes = read_input(path)?;
            Ok(format_output(&sha256_hex(&bytes), path))
        })
        .collect()
}

fn collect_files(
    root: &Path,
    extension: Option<&str>,
    paths: &mut Vec<PathBuf>,
) -> Result<(), String> {
    for entry in fs::read_dir(root)
        .map_err(|error| format!("failed to read directory '{}': {error}", root.display()))?
    {
        let path = entry
            .map_err(|error| format!("failed to read directory entry: {error}"))?
            .path();
        if path.is_dir() {
            collect_files(&path, extension, paths)?;
        } else if extension.is_none_or(|extension| {
            path.extension().and_then(|value| value.to_str()) == Some(extension)
        }) {
            paths.push(path);
        }
    }
    Ok(())
}

fn verify_manifest(path: &str) -> Result<String, String> {
    let manifest = fs::read_to_string(path)
        .map_err(|error| format!("failed to read manifest '{path}': {error}"))?;
    let mut verified = 0;

    for (line_number, line) in manifest.lines().enumerate() {
        if line.trim().is_empty() {
            continue;
        }
        let (expected, file_path) = line.split_once("  ").ok_or_else(|| {
            format!(
                "manifest line {} must use '<digest>  <path>'",
                line_number + 1
            )
        })?;
        if expected.len() != 64 || !expected.bytes().all(|byte| byte.is_ascii_hexdigit()) {
            return Err(format!(
                "manifest line {} has an invalid SHA-256 digest",
                line_number + 1
            ));
        }

        let actual = sha256_hex(&read_input(file_path)?);
        if actual != expected.to_ascii_lowercase() {
            return Err(format!(
                "checksum mismatch for '{file_path}': expected {expected}, found {actual}"
            ));
        }
        verified += 1;
    }

    Ok(format!("verified {verified} file(s)\n"))
}

fn format_output(digest: &str, path: &str) -> String {
    format!("{digest}  {path}\n")
}

fn read_input(path: &str) -> Result<Vec<u8>, String> {
    if path == "-" {
        let mut input = Vec::new();
        io::stdin()
            .read_to_end(&mut input)
            .map_err(|error| format!("failed to read standard input: {error}"))?;
        return Ok(input);
    }

    fs::read(path).map_err(|error| format!("failed to read '{path}': {error}"))
}

fn sha256_hex(input: &[u8]) -> String {
    let digest = sha256(input);
    digest.iter().map(|byte| format!("{byte:02x}")).collect()
}

fn sha256(input: &[u8]) -> [u8; 32] {
    const INITIAL_STATE: [u32; 8] = [
        0x6A09_E667,
        0xBB67_AE85,
        0x3C6E_F372,
        0xA54F_F53A,
        0x510E_527F,
        0x9B05_688C,
        0x1F83_D9AB,
        0x5BE0_CD19,
    ];
    const ROUND_CONSTANTS: [u32; 64] = [
        0x428A_2F98,
        0x7137_4491,
        0xB5C0_FBCF,
        0xE9B5_DBA5,
        0x3956_C25B,
        0x59F1_11F1,
        0x923F_82A4,
        0xAB1C_5ED5,
        0xD807_AA98,
        0x1283_5B01,
        0x2431_85BE,
        0x550C_7DC3,
        0x72BE_5D74,
        0x80DE_B1FE,
        0x9BDC_06A7,
        0xC19B_F174,
        0xE49B_69C1,
        0xEFBE_4786,
        0x0FC1_9DC6,
        0x240C_A1CC,
        0x2DE9_2C6F,
        0x4A74_84AA,
        0x5CB0_A9DC,
        0x76F9_88DA,
        0x983E_5152,
        0xA831_C66D,
        0xB003_27C8,
        0xBF59_7FC7,
        0xC6E0_0BF3,
        0xD5A7_9147,
        0x06CA_6351,
        0x1429_2967,
        0x27B7_0A85,
        0x2E1B_2138,
        0x4D2C_6DFC,
        0x5338_0D13,
        0x650A_7354,
        0x766A_0ABB,
        0x81C2_C92E,
        0x9272_2C85,
        0xA2BF_E8A1,
        0xA81A_664B,
        0xC24B_8B70,
        0xC76C_51A3,
        0xD192_E819,
        0xD699_0624,
        0xF40E_3585,
        0x106A_A070,
        0x19A4_C116,
        0x1E37_6C08,
        0x2748_774C,
        0x34B0_BCB5,
        0x391C_0CB3,
        0x4ED8_AA4A,
        0x5B9C_CA4F,
        0x682E_6FF3,
        0x748F_82EE,
        0x78A5_636F,
        0x84C8_7814,
        0x8CC7_0208,
        0x90BE_FFFA,
        0xA450_6CEB,
        0xBEF9_A3F7,
        0xC671_78F2,
    ];

    let mut message = input.to_vec();
    let bit_length = (message.len() as u64).wrapping_mul(8);
    message.push(0x80);
    while message.len() % 64 != 56 {
        message.push(0);
    }
    message.extend_from_slice(&bit_length.to_be_bytes());

    let mut state = INITIAL_STATE;
    for chunk in message.chunks_exact(64) {
        let mut words = [0u32; 64];
        for (index, word) in words[..16].iter_mut().enumerate() {
            let start = index * 4;
            *word = u32::from_be_bytes([
                chunk[start],
                chunk[start + 1],
                chunk[start + 2],
                chunk[start + 3],
            ]);
        }
        for index in 16..64 {
            let small_sigma0 = words[index - 15].rotate_right(7)
                ^ words[index - 15].rotate_right(18)
                ^ (words[index - 15] >> 3);
            let small_sigma1 = words[index - 2].rotate_right(17)
                ^ words[index - 2].rotate_right(19)
                ^ (words[index - 2] >> 10);
            words[index] = words[index - 16]
                .wrapping_add(small_sigma0)
                .wrapping_add(words[index - 7])
                .wrapping_add(small_sigma1);
        }

        let (mut a, mut b, mut c, mut d, mut e, mut f, mut g, mut h) = (
            state[0], state[1], state[2], state[3], state[4], state[5], state[6], state[7],
        );
        for index in 0..64 {
            let big_sigma1 = e.rotate_right(6) ^ e.rotate_right(11) ^ e.rotate_right(25);
            let choose = (e & f) ^ ((!e) & g);
            let temporary1 = h
                .wrapping_add(big_sigma1)
                .wrapping_add(choose)
                .wrapping_add(ROUND_CONSTANTS[index])
                .wrapping_add(words[index]);
            let big_sigma0 = a.rotate_right(2) ^ a.rotate_right(13) ^ a.rotate_right(22);
            let majority = (a & b) ^ (a & c) ^ (b & c);
            let temporary2 = big_sigma0.wrapping_add(majority);
            (h, g, f, e, d, c, b, a) = (
                g,
                f,
                e,
                d.wrapping_add(temporary1),
                c,
                b,
                a,
                temporary1.wrapping_add(temporary2),
            );
        }

        for (value, addition) in state.iter_mut().zip([a, b, c, d, e, f, g, h]) {
            *value = value.wrapping_add(addition);
        }
    }

    let mut digest = [0u8; 32];
    for (index, word) in state.iter().enumerate() {
        digest[index * 4..index * 4 + 4].copy_from_slice(&word.to_be_bytes());
    }
    digest
}

#[cfg(test)]
mod tests {
    use super::{sha256, sha256_hex, verify_manifest};

    #[test]
    fn hashes_standard_vectors() {
        assert_eq!(
            sha256_hex(b""),
            "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
        );
        assert_eq!(
            sha256_hex(b"abc"),
            "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad"
        );
    }

    #[test]
    fn produces_a_fixed_size_digest() {
        assert_eq!(sha256(b"checksum").len(), 32);
    }

    #[test]
    fn formats_script_friendly_output_for_stdin() {
        assert_eq!(
            super::format_output(
                "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad",
                "-"
            ),
            "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad  -\n"
        );
    }

    #[test]
    fn verifies_matching_manifest_entries() {
        let directory = tempfile::tempdir().unwrap();
        let file = directory.path().join("sample.txt");
        let manifest = directory.path().join("checksums.txt");
        std::fs::write(&file, b"abc").unwrap();
        std::fs::write(
            &manifest,
            format!("{}  {}\n", sha256_hex(b"abc"), file.display()),
        )
        .unwrap();

        assert_eq!(
            verify_manifest(manifest.to_str().unwrap()),
            Ok("verified 1 file(s)\n".to_string())
        );
    }

    #[test]
    fn reports_manifest_mismatches() {
        let directory = tempfile::tempdir().unwrap();
        let file = directory.path().join("sample.txt");
        let manifest = directory.path().join("checksums.txt");
        std::fs::write(&file, b"abc").unwrap();
        std::fs::write(
            &manifest,
            format!("{}  {}\n", "0".repeat(64), file.display()),
        )
        .unwrap();

        assert!(verify_manifest(manifest.to_str().unwrap())
            .unwrap_err()
            .contains("checksum mismatch"));
    }

    #[test]
    fn collects_files_in_sorted_extension_filtered_order() {
        let directory = tempfile::tempdir().unwrap();
        let nested = directory.path().join("nested");
        std::fs::create_dir(&nested).unwrap();
        std::fs::write(directory.path().join("z.txt"), b"z").unwrap();
        std::fs::write(directory.path().join("skip.bin"), b"skip").unwrap();
        std::fs::write(nested.join("a.txt"), b"a").unwrap();

        let mut paths = Vec::new();
        super::collect_files(directory.path(), Some("txt"), &mut paths).unwrap();
        paths.sort();

        assert_eq!(
            paths,
            vec![
                directory.path().join("nested/a.txt"),
                directory.path().join("z.txt")
            ]
        );
    }
}
