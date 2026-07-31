# TODO: simple_grep (⭐ 2/10)

## Usage

```bash
cargo run --bin simple_grep -- "pattern" file.txt
cargo run --bin simple_grep -- "pattern" file.txt --case-sensitive
cargo test --bin simple_grep
```

## Milestones

- [ ] Accept pattern and filename arguments.
- [ ] Search for pattern in file (case-insensitive by default).
- [ ] Print matching lines with line numbers.
- [ ] Add --case-sensitive flag.
- [ ] Handle file errors gracefully.
- [ ] Add tests for search logic.

## Extra

- [ ] Add --count flag to show match count only.
- [ ] Add --invert flag to show non-matching lines.

## Tips

- Use String methods like `.contains()` for search.
- Use `.to_lowercase()` for case-insensitive matching.
- Test with fixture files.
