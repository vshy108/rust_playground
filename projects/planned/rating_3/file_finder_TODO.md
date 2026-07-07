# TODO: file_finder (⭐ 3/10)

## Usage

```bash
cargo run --bin file_finder
cargo test --bin file_finder
```

## Milestones

- [ ] Walk directories recursively from a root path.
- [ ] Match files by name pattern or extension.
- [ ] Add hidden-file or depth-limit options.
- [ ] Print stable, easy-to-parse results.
- [ ] Add tests for traversal and filtering logic.

## Extra

- [ ] Add simple content-grep mode.

## Tips

- Traversal and filtering should be separate units.
- Stable output ordering helps testability and scripting.
