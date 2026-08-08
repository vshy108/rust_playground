# TODO: notes_cli (⭐ 3/10)

## Usage

```bash
cargo run --bin notes_cli
cargo test --bin notes_cli
```

## Milestones

- [x] Store simple note entries in a local file.
- [x] Add list and add subcommands.
- [x] Support note IDs or timestamps.
- [x] Add delete or complete behavior.
- [x] Add tests for storage round-trips.

## Extra

- [x] Add tag-based filtering.

## Status

Completed

## Specification

### Goal

Manage local notes with add, list, complete, delete, and tag-filter operations.

### Non-goals

- Multi-user synchronization
- Rich-text editing
- Encryption or network storage

### Inputs and outputs

- Input: JSON storage path and note subcommands
- Output: stable human-readable status lines and note listings

### Errors and limits

- Report invalid IDs, missing notes, malformed storage, and file errors clearly.
- Preserve note IDs and tags across storage round-trips.

### Acceptance criteria

- [x] Local persistence and add/list commands work.
- [x] Complete/delete and tag filtering work.
- [x] Storage round-trip and rendering tests pass.

## Change record

- Scope: implemented JSON-backed note persistence and all requested commands.
- Tests added: storage round-trip, completion rendering, and tag filtering coverage.
- Commands run: focused `rustfmt`, `cargo check`, `cargo test`, and `cargo clippy`.

## Tips

- Keep storage format simple so the project stays below the difficulty target.
- Separate command parsing from note persistence.
