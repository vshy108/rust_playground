# TODO: mini_grep_cli

## 1. Basic Grep

- [x] Read pattern and filename from CLI args (exit with usage message if missing).
- [x] Read file contents with `fs::read_to_string`.
- [x] Split contents into lines with `.lines()`.
- [x] Filter lines that contain the pattern with `.filter()`.
- [x] Print matching lines to stdout with line numbers.

Acceptance checks:

```bash
echo -e 'hello world\ngoodbye\nhello rust' > /tmp/logs.txt
cargo run --bin rgrep -- hello /tmp/logs.txt
# expected: lines containing "hello" with line numbers

cargo run --bin rgrep -- notfound /tmp/logs.txt
# expected: no output, exit 0

cargo run --bin rgrep
# expected: usage message, non-zero exit
```

## 2. Tests

- [ ] Matching a pattern returns only lines that contain it.
- [ ] Non-matching pattern returns empty results.
- [ ] Line numbers are correct (1-based).

Acceptance check:

```bash
cargo test --bin rgrep
```

## 3. Extra

- [ ] Regex support: accept a regex pattern instead of a plain string (use `regex` crate).
