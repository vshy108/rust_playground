# TODO: url_codec (⭐ 2/10)

## Usage

```bash
cargo run --bin url_codec
cargo test --bin url_codec
```

## Milestones

- [ ] Encode URL components from CLI input.
- [ ] Decode percent-encoded strings.
- [ ] Add explicit encode and decode modes.
- [ ] Report malformed escape sequences clearly.
- [ ] Add tests for spaces, symbols, and invalid inputs.

## Extra

- [ ] Add query-string key/value formatting helpers.

## Tips

- Keep component encoding separate from full-URL parsing.
- Error messages matter because malformed percent escapes are common.
