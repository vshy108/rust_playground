# TODO: duplicate_line_filter (⭐ 3/10)

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
