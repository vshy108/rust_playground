# TODO: slug_generator (⭐ 2/10)

## Usage

```bash
cargo run --bin slug_generator
cargo test --bin slug_generator
```

## Milestones

- [ ] Accept text from args or stdin.
- [ ] Lowercase and normalize separator characters.
- [ ] Collapse duplicate separators.
- [ ] Trim separators from both ends.
- [ ] Add tests for punctuation-heavy inputs.

## Extra

- [ ] Add optional maximum slug length.

## Tips

- Split transformation steps so each one can be unit-tested.
- Decide how to treat non-ASCII characters before writing the CLI layer.
