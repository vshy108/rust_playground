# TODO: slug_generator (⭐ 2/10)

## Usage

```bash
cargo run --bin slug_generator
cargo test --bin slug_generator
```

## Milestones

- [x] Accept text from args or stdin.
- [x] Lowercase and normalize separator characters.
- [x] Collapse duplicate separators.
- [x] Trim separators from both ends.
- [x] Add tests for punctuation-heavy inputs.

## Extra

- [x] Add optional maximum slug length.

## Tips

- Split transformation steps so each one can be unit-tested.
- Decide how to treat non-ASCII characters before writing the CLI layer.
