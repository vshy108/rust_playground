# TODO: env_linter (⭐ 3/10)

## Usage

```bash
cargo run --bin env_linter
cargo test --bin env_linter
```

## Milestones

- [x] Parse `.env` style key-value files.
- [x] Detect common issues such as duplicate keys or invalid names.
- [x] Add missing-value or whitespace warning rules.
- [x] Implement human-readable and machine-readable output.
- [x] Add tests for rule triggering and parser edge cases.

## Extra

- [x] Add autofix suggestions for simple issues.

## Status

Completed

## Specification

### Goal

Lint `.env` key-value files for invalid names, duplicate keys, empty values, and
avoidable assignment whitespace.

### Non-goals

- Secret-value detection
- Expanding variables or interpreting shell syntax
- Rewriting files automatically

### Inputs and outputs

- Input: optional file or stdin, with optional `--json`
- Output: human-readable diagnostics or JSON issue records

### Errors and limits

- Preserve first-seen duplicate locations and provide simple fix suggestions.
- Report malformed lines without aborting the remaining file.

### Acceptance criteria

- [x] Parser and lint rules work.
- [x] Human and JSON output work.
- [x] Duplicate, invalid-name, empty-value, whitespace, and malformed-line rules work.
- [x] Autofix suggestions and focused tests work.

## Change record

- Scope: implemented the complete `.env` parser and linter workflow.
- Tests added: rule-triggering, suggestions, comments, and name validation tests.
- Commands run: focused `rustfmt`, `cargo check`, `cargo test`, and `cargo clippy`.

## Tips

- Keep parsing separate from lint rule evaluation.
- Duplicate-key handling should preserve first-seen location info.
