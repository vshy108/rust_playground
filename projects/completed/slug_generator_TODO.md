# TODO: slug_generator (⭐ 2/10)

## Status

Completed

## Specification

### Goal

Convert input text into a predictable lowercase ASCII URL slug.

### Non-goals

- Transliteration of non-ASCII characters
- Unicode locale-specific normalization
- Collision detection or slug persistence

### Inputs and outputs

- Input: text from arguments or standard input, with an optional maximum length
- Output: lowercase words separated by single hyphens

### Errors and limits

- Reject invalid maximum lengths and trim the result without trailing separators.
- Remove unsupported punctuation and non-ASCII characters predictably.

### Acceptance criteria

- [x] Whitespace and punctuation normalize to separators.
- [x] Duplicate and edge separators are removed.
- [x] ASCII-only output and maximum length work.
- [x] Punctuation-heavy tests pass.

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

## Change record

- Scope: verified and moved the completed slug generator to `projects/completed/`.
- Tests added: existing tests cover punctuation, separators, ASCII behavior, length limits, and invalid options.
- Commands run: `rustfmt`, focused `cargo test`, `cargo check`, and `cargo clippy`.
- Follow-up: rating-2 projects are complete; continue with rating 3.
