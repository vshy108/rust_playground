# TODO: url_codec (⭐ 2/10)

## Usage

```bash
cargo run --bin url_codec
cargo test --bin url_codec
```

## Milestones

- [x] Encode URL components from CLI input.
- [x] Decode percent-encoded strings.
- [x] Add explicit encode and decode modes.
- [x] Report malformed escape sequences clearly.
- [x] Add tests for spaces, symbols, and invalid inputs.

## Extra

- [x] Add query-string key/value formatting helpers.

## Tips

- Keep component encoding separate from full-URL parsing.
- Error messages matter because malformed percent escapes are common.
