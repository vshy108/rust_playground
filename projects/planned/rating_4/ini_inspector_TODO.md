# TODO: ini_inspector (⭐ 4/10)

## Usage

```bash
cargo run --bin ini_inspector
cargo test --bin ini_inspector
```

## Milestones

- [ ] Parse sections and key/value pairs from an INI file.
- [ ] Print a readable summary of discovered sections.
- [ ] Support querying one section or key path.
- [ ] Report duplicate keys or malformed lines.
- [ ] Add fixture-based parser tests.

## Extra

- [ ] Add rewrite support with normalized formatting.

## Tips

- Decide early how strict the parser should be with comments and whitespace.
- A small internal data model will simplify querying and diagnostics.
