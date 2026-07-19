# TODO: wc_clone (⭐ 2/10)

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
