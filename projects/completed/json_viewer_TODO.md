# TODO: json_viewer (⭐ 3/10)

## Usage

```bash
cargo run --bin json_viewer
cargo test --bin json_viewer
```

## Milestones

- [x] Read JSON input from file or stdin.
- [x] Pretty-print nested structures with indentation.
- [x] Add compact and pretty output modes.
- [x] Surface parse errors with useful context.
- [x] Add tests for formatting and malformed input handling.

## Extra

- [x] Add simple dotted path lookup mode.

## Status

Completed

## Specification

### Goal

Read JSON from a file or stdin, render it predictably, and optionally print a
nested value selected by a dotted path.

### Non-goals

- JSON mutation or filtering expressions
- Stable sorting of object keys beyond serde_json behavior
- Network input

### Inputs and outputs

- Input: optional file, `--compact`, and `--get PATH`
- Output: pretty JSON by default, compact JSON when requested

### Errors and limits

- Report file, parse, option, and missing-path errors with useful context.
- Support object keys and numeric array segments in lookup paths.

### Acceptance criteria

- [x] File and stdin input work.
- [x] Pretty and compact rendering work.
- [x] Nested object/array lookup works.
- [x] Invalid JSON and missing paths return clear errors.
- [x] Focused tests and clippy pass.

## Change record

- Scope: implemented the complete JSON viewing and lookup workflow.
- Tests added: option parsing, nested lookup, and missing-path coverage.
- Commands run: focused `rustfmt`, `cargo check`, `cargo test`, and `cargo clippy`.

## Tips

- Parsing and rendering should stay separate.
- Stable key-order expectations matter if tests use snapshots.
