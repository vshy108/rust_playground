# TODO: wc_clone (⭐ 2/10)

## Status

Completed

## Specification

### Goal

Count lines, words, bytes, or UTF-8 characters from a file or standard input.

### Non-goals

- Recursive directory traversal
- Locale-specific word counting
- Streaming output beyond one input source

### Inputs and outputs

- Input: optional `-l`, `-w`, `-c`, or character-count flags and an optional path
- Output: selected counts in the standard layout

### Errors and limits

- Report unreadable files and invalid options clearly.
- Preserve byte counts separately from UTF-8 character counts.

### Acceptance criteria

- [x] File and stdin input work.
- [x] Line, word, byte, and character counts work.
- [x] Combined flags and output formatting are stable.
- [x] Fixture-based tests pass.

## Usage

```bash
cargo run --bin wc_clone
cargo test --bin wc_clone
```

## Milestones

- [x] Read from a file path or stdin.
- [x] Count lines, words, and bytes.
- [x] Match a simple default output layout.
- [x] Add per-flag output selection.
- [x] Add fixture-based tests for counts.

## Extra

- [x] Add character counting for UTF-8 text.

## Tips

- Decide early whether to stream bytes or read whole input.
- Keep counting logic separate from CLI formatting.

## Change record

- Scope: verified and moved the completed counter to `projects/completed/`.
- Tests added: existing tests cover fixture counts, UTF-8 characters, flags, and formatting.
- Commands run: `rustfmt`, focused `cargo test`, `cargo check`, and `cargo clippy`.
- Follow-up: rating-2 projects are complete; continue with rating 3.
