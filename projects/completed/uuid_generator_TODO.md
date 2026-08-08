# TODO: uuid_generator (⭐ 2/10)

## Status

Completed

## Specification

### Goal

Generate random or namespace-based UUIDs from a small command-line interface.

### Non-goals

- Persistent UUID storage
- Network access or external services
- UUID versions other than random version 4 and namespace version 5

### Inputs and outputs

- Input: optional `--count`, `--format plain|uppercase|urn`, and
  `--namespace UUID NAME` options
- Output: one formatted UUID per line

### Errors and limits

- Reject zero or invalid counts, unknown formats/options, malformed namespace
  UUIDs, and incompatible count/namespace combinations.
- Use operating-system randomness for normal generation and preserve UUID
  version and RFC variant bits.

### Acceptance criteria

- [x] Random version-4 UUID generation works.
- [x] Count and plain/uppercase/URN output modes work.
- [x] Namespace version-5 UUID generation is deterministic.
- [x] Invalid options and UUID input return clear errors.
- [x] Formatting, shape, and namespace tests pass.

## Usage

```bash
cargo run --bin uuid_generator
cargo test --bin uuid_generator
```

## Milestones

- [x] Generate UUIDs from a simple CLI.
- [x] Add count or batch generation mode.
- [x] Support a couple of output formats such as plain or uppercase.
- [x] Validate output shape in tests.
- [x] Add tests for formatting and count behavior.

## Extra

- [x] Add namespace-based deterministic UUID mode.

## Tips

- Output formatting is the main behavior to pin down.
- Keep generation logic and CLI formatting separate.

## Change record

- Scope: verified the existing UUID implementation and moved it to the
  completed-projects path.
- Assumptions: the fallback random source is retained for environments without
  `/dev/urandom`.
- Tests added: no new tests; existing tests cover version/variant bits,
  formatting, parsing, deterministic namespace output, and invalid input.
- Commands run: `rustfmt projects/completed/uuid_generator.rs`, `cargo test
  --bin uuid_generator`, and `cargo clippy --bin uuid_generator --all-features
  -- -D warnings`.
- Known limitations: random fallback is best-effort and intended only when the
  operating-system source is unavailable.
- Follow-up: begin the next unfinished rating-2 project, `gitignore_gen`.
