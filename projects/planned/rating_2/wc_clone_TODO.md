# TODO: wc_clone (⭐ 2/10)

## Usage

```bash
cargo run --bin wc_clone
cargo test --bin wc_clone
```

## Milestones

- [ ] Read from a file path or stdin.
- [ ] Count lines, words, and bytes.
- [ ] Match a simple default output layout.
- [ ] Add per-flag output selection.
- [ ] Add fixture-based tests for counts.

## Extra

- [ ] Add character counting for UTF-8 text.

## Tips

- Decide early whether to stream bytes or read whole input.
- Keep counting logic separate from CLI formatting.
