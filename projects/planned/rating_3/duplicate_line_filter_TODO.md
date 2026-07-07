# TODO: duplicate_line_filter (⭐ 3/10)

## Usage

```bash
cargo run --bin duplicate_line_filter
cargo test --bin duplicate_line_filter
```

## Milestones

- [ ] Read lines from stdin or file input.
- [ ] Track seen lines and filter duplicates.
- [ ] Add options for case sensitivity or whitespace normalization.
- [ ] Implement counts or keep-first/keep-last behavior.
- [ ] Add tests for duplicate detection and normalization options.

## Extra

- [ ] Add streaming statistics output.

## Tips

- Decide early whether equality is byte-level or normalized text.
- Streaming behavior should not require loading unrelated metadata.
- Simple fixtures are enough to catch most regressions here.
