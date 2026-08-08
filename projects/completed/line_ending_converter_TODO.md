# TODO: line_ending_converter (⭐ 3/10)

## Status

Completed

## Specification

### Goal

Detect and convert LF/CRLF line endings for stdin, files, or filtered directories.

### Non-goals

- Binary-file conversion
- Recursive deletion or renaming
- Implicit modification without an explicit mode

### Inputs and outputs

- Input: `lf` or `crlf` target with stdin, in-place, or directory mode
- Output: converted content or stable conversion status lines

### Errors and limits

- Preserve trailing newlines and leave matching files unchanged.
- Report unreadable paths and invalid modes clearly.

### Acceptance criteria

- [x] LF, CRLF, and mixed content are detected.
- [x] Conversion preserves content and trailing newlines.
- [x] In-place and extension-filtered directory modes work.
- [x] Matching files are not rewritten.

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

## Change record

- Scope: used newline detection to skip matching files and moved the completed converter to `projects/completed/`.
- Tests added: existing tests cover detection, conversion, and unchanged content.
- Commands run: focused `rustfmt`, `cargo check`, `cargo test`, and `cargo clippy`.
- Follow-up: continue with the next incomplete rating-3 project.
