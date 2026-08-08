# TODO: file_finder (⭐ 3/10)

## Usage

```bash
cargo run --bin file_finder
cargo test --bin file_finder
```

## Milestones

- [x] Walk directories recursively from a root path.
- [x] Match files by name pattern or extension.
- [x] Add hidden-file or depth-limit options.
- [x] Print stable, easy-to-parse results.
- [x] Add tests for traversal and filtering logic.

## Extra

- [x] Add simple content-grep mode.

## Status

Completed

## Specification

### Goal

Find files recursively with deterministic output and simple name, extension,
depth, hidden-file, and content filters.

### Non-goals

- Full regular-expression search
- Following symlinks
- Parallel traversal

### Inputs and outputs

- Input: root path and optional filters
- Output: one matching path per line, sorted lexicographically

### Errors and limits

- Report unreadable directories/files clearly.
- Skip hidden entries unless explicitly requested.

### Acceptance criteria

- [x] Recursive traversal and stable output work.
- [x] Name, extension, hidden, depth, and content filters work.
- [x] Wildcard matching and traversal tests pass.

## Change record

- Scope: implemented recursive deterministic file discovery and filtering.
- Tests added: wildcard, sorted extension filtering, and option parsing coverage.
- Commands run: focused `rustfmt`, `cargo check`, `cargo test`, and `cargo clippy`.

## Tips

- Traversal and filtering should be separate units.
- Stable output ordering helps testability and scripting.
