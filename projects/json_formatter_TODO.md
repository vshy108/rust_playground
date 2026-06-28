# TODO: json_formatter_cli

## Usage

```bash
cargo run --bin jsonfmt -- fixtures/compact.json        # pretty-print file
cargo run --bin jsonfmt                                 # read from stdin
cargo run --bin jsonfmt -- --check fixtures/compact.json  # validate only
cargo run --bin jsonfmt -- --help
cargo test --bin jsonfmt
```

## 1. Basic Formatter

- [x] Read filename from CLI args (exit with usage message if missing).
- [x] Read file contents with `fs::read_to_string`.
- [x] Deserialize into `serde_json::Value` with `serde_json::from_str`.
- [x] Re-serialize with `serde_json::to_string_pretty` and print to stdout.
- [x] Propagate errors with `?` and `Box<dyn Error>` on `main`.

Acceptance checks:

```bash
cargo run --bin jsonfmt -- fixtures/compact.json
# expected: pretty-printed JSON with indentation

cargo run --bin jsonfmt -- fixtures/bad.json
# expected: error message, non-zero exit
```

## 2. Tests

- [x] Parsing valid JSON object returns `Ok`.
- [x] Parsing invalid JSON returns `Err`.
- [x] Pretty-printed output contains newlines and indentation.

Acceptance check:

```bash
cargo test --bin jsonfmt
```

## 3. Extra

- [x] Pretty print (already using `to_string_pretty` — verify indentation is 2 spaces).
- [x] Validate-only mode: `--check` flag exits 0 if valid, 1 if invalid, prints nothing.
- [x] Read from stdin when no filename is given (`-` or no arg).
