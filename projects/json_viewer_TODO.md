# TODO: json_viewer (⭐ 3/10)

## Usage

```bash
cargo run --bin json_viewer
cargo test --bin json_viewer
```

## Milestones

- [ ] Read JSON input from file or stdin.
- [ ] Pretty-print nested structures with indentation.
- [ ] Add compact and pretty output modes.
- [ ] Surface parse errors with useful context.
- [ ] Add tests for formatting and malformed input handling.

## Extra

- [ ] Add simple path lookup mode.

## Tips

- Parsing and rendering should stay separate.
- Stable key-order expectations matter if tests use snapshots.
