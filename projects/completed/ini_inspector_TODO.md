# TODO: ini_inspector (⭐ 4/10)

## Usage

```bash
cargo run --bin ini_inspector
cargo test --bin ini_inspector
```

## Milestones

- [x] Parse sections and key/value pairs from an INI file.
- [x] Print a readable summary of discovered sections.
- [x] Support querying one section or key path.
- [x] Report duplicate keys or malformed lines.
- [x] Add fixture-based parser tests.

## Extra

- [x] Add rewrite support with normalized formatting.

## Status

Completed.

## Specification

- Goal: inspect small INI files with deterministic parsing and diagnostics.
- Inputs: an INI file with optional summary, SECTION.KEY query, or rewrite operation.
- Output: section summaries, queried values, or normalized key/value output.
- Errors: reject malformed sections, missing separators, empty keys, duplicate keys,
  missing queries, and file I/O failures.
- Non-goals: interpolation, includes, typed values, and preserving comments in rewritten output.
- Acceptance: fixture, duplicate, malformed-line, query, and normalization tests pass with strict Clippy enabled.

## Change record

- Implemented the parser, diagnostics, summary/query commands, normalized rewrite support, and fixture-style tests.

## Tips

- Decide early how strict the parser should be with comments and whitespace.
- A small internal data model will simplify querying and diagnostics.
