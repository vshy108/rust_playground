# TODO: ascii_table (⭐ 2/10)

## Usage

```bash
cargo run --bin ascii_table
cargo test --bin ascii_table
```

## Milestones

- [ ] Print the standard ASCII range in columns.
- [ ] Show decimal, hex, and printable character forms.
- [ ] Handle control characters with readable labels.
- [ ] Support a numeric range filter.
- [ ] Add snapshot-like tests for output formatting.

## Extra

- [ ] Add an extended Latin-1 mode.

## Tips

- Model each row first, then render the table from data.
- Control-character labels are easier to test than raw terminal behavior.
