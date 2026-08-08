# TODO: ascii_table (⭐ 2/10)

## Status

Completed

## Specification

### Goal

Render ASCII reference rows with decimal, hexadecimal, and readable character forms.

### Non-goals

- Interactive terminal navigation
- Unicode code-point tables
- Custom color themes

### Inputs and outputs

- Input: optional decimal or hexadecimal range and extended Latin-1 mode
- Output: stable column-aligned table rows

### Errors and limits

- Reject reversed or malformed ranges.
- Label control characters instead of emitting raw terminal controls.

### Acceptance criteria

- [x] Standard ASCII rows and control labels render correctly.
- [x] Decimal and hexadecimal ranges work.
- [x] Extended Latin-1 mode works.
- [x] Snapshot-like formatting tests pass.

## Usage

```bash
cargo run --bin ascii_table
cargo test --bin ascii_table
```

## Milestones

- [x] Print the standard ASCII range in columns.
- [x] Show decimal, hex, and printable character forms.
- [x] Handle control characters with readable labels.
- [x] Support a numeric range filter.
- [x] Add snapshot-like tests for output formatting.

## Extra

- [x] Add an extended Latin-1 mode.

## Tips

- Model each row first, then render the table from data.
- Control-character labels are easier to test than raw terminal behavior.

## Change record

- Scope: verified and moved the completed ASCII table generator to `projects/completed/`.
- Tests added: existing tests cover labels, ranges, formatting, and extended rows.
- Commands run: `rustfmt`, focused `cargo test`, `cargo check`, and `cargo clippy`.
- Follow-up: rating-2 projects are complete; continue with rating 3.
