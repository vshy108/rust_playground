# TODO: duplicate_line_filter (⭐ 3/10)

## Status

Completed

## Specification

### Goal

Filter duplicate lines from stdin or a file with configurable normalization and
streaming statistics.

### Non-goals

- Fuzzy matching
- Sorting input lines
- Loading unrelated metadata

### Inputs and outputs

- Input: lines plus case, trim, keep-last, counts, and statistics options
- Output: selected lines and optional occurrence/statistics information

### Errors and limits

- Report invalid options and unreadable files clearly.
- Preserve first/last ordering semantics explicitly.

### Acceptance criteria

- [x] Duplicate detection and normalization work.
- [x] Keep-first, keep-last, counts, and statistics modes work.
- [x] File/stdin input is supported.
- [x] Duplicate and normalization tests pass.

## Usage

```bash
cargo run --bin duplicate_line_filter
cargo test --bin duplicate_line_filter
```

## Milestones

- [x] Read lines from stdin or file input.
- [x] Track seen lines and filter duplicates.
- [x] Add options for case sensitivity or whitespace normalization.
- [x] Implement counts or keep-first/keep-last behavior.
- [x] Add tests for duplicate detection and normalization options.

## Extra

- [x] Add streaming statistics output.

## Tips

- Decide early whether equality is byte-level or normalized text.
- Streaming behavior should not require loading unrelated metadata.
- Simple fixtures are enough to catch most regressions here.

## Change record

- Scope: fixed a strict-clippy warning and moved the completed filter to `projects/completed/`.
- Tests added: existing tests cover duplicate behavior, normalization, keep-last, and counts.
- Commands run: focused `rustfmt`, `cargo check`, `cargo test`, and `cargo clippy`.
- Follow-up: continue with the next incomplete rating-3 project.
