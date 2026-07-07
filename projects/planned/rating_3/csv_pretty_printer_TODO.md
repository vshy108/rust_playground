# TODO: csv_pretty_printer (⭐ 3/10)

## Usage

```bash
cargo run --bin csv_pretty_printer
cargo test --bin csv_pretty_printer
```

## Milestones

- [ ] Read CSV input from file or stdin.
- [ ] Compute column widths and render aligned tables.
- [ ] Add header/no-header modes.
- [ ] Handle quoted values and uneven rows gracefully.
- [ ] Add tests for alignment and parse-edge cases.

## Extra

- [ ] Add column selection or truncation rules.

## Tips

- Parsing and rendering are separate concerns.
- Width calculations should be deterministic and test-friendly.
