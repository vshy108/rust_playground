# TODO: notes_cli (⭐ 3/10)

## Usage

```bash
cargo run --bin notes_cli
cargo test --bin notes_cli
```

## Milestones

- [ ] Store simple note entries in a local file.
- [ ] Add list and add subcommands.
- [ ] Support note IDs or timestamps.
- [ ] Add delete or complete behavior.
- [ ] Add tests for storage round-trips.

## Extra

- [ ] Add tag-based filtering.

## Tips

- Keep storage format simple so the project stays below the difficulty target.
- Separate command parsing from note persistence.
