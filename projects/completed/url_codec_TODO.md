# TODO: url_codec (⭐ 2/10)

## Status

Completed

## Specification

### Goal

Encode and decode URL components and format query-string key/value pairs.

### Non-goals

- Full URL parsing or normalization
- Network requests
- Form-body plus-space semantics

### Inputs and outputs

- Input: explicit encode/decode mode and component or query-pair text
- Output: percent-encoded or decoded component text

### Errors and limits

- Reject malformed percent escapes and incomplete byte sequences.
- Preserve UTF-8 bytes during encoding and decoding.

### Acceptance criteria

- [x] Spaces, symbols, and UTF-8 components encode correctly.
- [x] Percent escapes decode correctly.
- [x] Malformed escapes return clear errors.
- [x] Query pairs format as encoded components.

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

## Change record

- Scope: verified and moved the completed URL codec to `projects/completed/`.
- Tests added: existing tests cover encoding, decoding, malformed escapes, and query formatting.
- Commands run: `rustfmt`, focused `cargo test`, `cargo check`, and `cargo clippy`.
- Follow-up: rating-2 projects are complete; continue with rating 3.
