# TODO: ascii_table (⭐ 2/10)

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
