# TODO: line_ending_converter (⭐ 3/10)

## Usage

```bash
cargo run --bin line_ending_converter
cargo test --bin line_ending_converter
```

## Milestones

- [x] Detect LF versus CRLF input content.
- [x] Convert text to a requested target line ending.
- [x] Support stdout and in-place rewrite modes.
- [x] Preserve files that already match the target.
- [x] Add tests for mixed-ending fixtures.

## Extra

- [x] Add directory mode with extension filtering.

## Tips

- Treat newline normalization as a pure string transform first.
- Be explicit about whether a trailing newline should be preserved.
