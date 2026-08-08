# TODO: gitignore_gen (⭐ 2/10)

## Status

Completed

## Specification

### Goal

Compose useful `.gitignore` content from named project presets and optional
custom patterns.

### Non-goals

- Fetching presets from the network
- Editing existing files in place without an explicit output path
- Parsing or validating `.gitignore` semantics beyond preserving patterns

### Inputs and outputs

- Input: comma-separated presets, optional `--custom` patterns, and optional
  `--output PATH` or `--stdout`
- Output: ordered, de-duplicated ignore patterns with a trailing newline

### Errors and limits

- Reject unknown options, missing option values, unknown presets, and empty
  input.
- Preserve first occurrence ordering when preset patterns overlap.

### Acceptance criteria

- [x] Multiple presets compose in requested order.
- [x] Duplicate patterns are removed without reordering unique entries.
- [x] Custom patterns can be merged.
- [x] Output can be printed or explicitly written to a file.
- [x] Invalid input returns clear errors.

## Usage

```bash
cargo run --bin gitignore_gen
cargo test --bin gitignore_gen
```

## Milestones

- [x] Parse one or more project-type presets from CLI input.
- [x] Combine matching ignore patterns into output text.
- [x] Remove duplicates while preserving useful ordering.
- [x] Add overwrite or print-to-stdout modes.
- [x] Add tests for preset composition and ordering.

## Extra

- [x] Add custom pattern merge support.

## Tips

- Preset data and rendering logic should stay separate.
- Ordering matters because humans will read the output.

## Change record

- Scope: verified the existing generator implementation and moved it to the
  completed-projects path.
- Assumptions: writing a file is opt-in through `--output`; default behavior is
  stdout output.
- Tests added: no new tests; existing tests cover composition, ordering,
  de-duplication, custom patterns, and unknown presets.
- Commands run: `rustfmt projects/completed/gitignore_gen.rs`, `cargo test --bin
  gitignore_gen`, and `cargo clippy --bin gitignore_gen --all-features -- -D
  warnings`.
- Known limitations: presets are static and intentionally small.
- Follow-up: begin the next unfinished rating-2 project, `color_preview`.
