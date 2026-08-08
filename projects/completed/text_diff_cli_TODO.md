# TODO: text_diff_cli (⭐ 4/10)

## Usage

```bash
cargo run --bin text_diff_cli
cargo test --bin text_diff_cli
```

## Milestones

- [x] Load two text inputs from files or stdin.
- [x] Implement line-by-line diff output.
- [x] Add markers for added, removed, and unchanged lines.
- [x] Handle newline and empty-file edge cases.
- [x] Add tests for diff rendering on representative fixtures.

## Extra

- [x] Add word-level highlighting inside changed lines.

## Status

Completed.

## Specification

- Goal: compare two text streams with deterministic line and word-level output.
- Inputs: two file paths, with a single dash meaning stdin.
- Output: unchanged lines use two spaces, additions use plus, removals use
  minus, and paired replacements highlight changed words.
- Errors: reject incorrect argument counts and report file or stdin failures.
- Non-goals: binary data, unified diff metadata, and patch application.
- Acceptance: LCS ordering, additions, removals, replacements, and empty-input
  tests pass with strict Clippy enabled.

## Change record

- Implemented file/stdin loading, LCS-based line comparison, stable rendering,
  word-level replacement highlighting, and focused edge-case tests.

## Tips

- Diff algorithm and output formatting should not be tightly coupled.
- Small fixture pairs give good coverage for this type of tool.
