# TODO: csv_pretty_printer (⭐ 3/10)

## Usage

```bash
cargo run --bin csv_pretty_printer
cargo test --bin csv_pretty_printer
```

## Milestones

- [x] Read CSV input from file or stdin.
- [x] Compute column widths and render aligned tables.
- [x] Add header/no-header modes.
- [x] Handle quoted values and uneven rows gracefully.
- [x] Add tests for alignment and parse-edge cases.

## Extra

- [x] Add column selection rules.

## Status

Completed

## Specification

### Goal

Parse CSV input and render deterministic, aligned terminal tables.

### Non-goals

- CSV export or mutation
- Type inference or sorting
- Truncation beyond explicit column selection

### Inputs and outputs

- Input: optional file, stdin, `--no-header`, and `--columns INDEX,...`
- Output: aligned table with header separator unless disabled

### Errors and limits

- Support quoted commas and escaped quotes.
- Reject unterminated quotes, invalid options, and missing selected columns.
- Pad uneven rows with empty cells.

### Acceptance criteria

- [x] File/stdin parsing and aligned rendering work.
- [x] Header and no-header modes work.
- [x] Quoted values and uneven rows are handled.
- [x] Column selection and parse-edge tests pass.

## Change record

- Scope: implemented the complete CSV parsing, rendering, and selection workflow.
- Tests added: quoted fields, escaped quotes, alignment, column selection, and malformed input coverage.
- Commands run: focused `rustfmt`, `cargo check`, `cargo test`, and `cargo clippy`.

## Tips

- Parsing and rendering are separate concerns.
- Width calculations should be deterministic and test-friendly.
