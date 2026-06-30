# TODO: line_ending_converter (⭐ 3/10)

## Usage

```bash
cargo run --bin line_ending_converter
cargo test --bin line_ending_converter
```

## Milestones

- [ ] Detect LF versus CRLF input content.
- [ ] Convert text to a requested target line ending.
- [ ] Support stdout and in-place rewrite modes.
- [ ] Preserve files that already match the target.
- [ ] Add tests for mixed-ending fixtures.

## Extra

- [ ] Add directory mode with extension filtering.

## Tips

- Treat newline normalization as a pure string transform first.
- Be explicit about whether a trailing newline should be preserved.
