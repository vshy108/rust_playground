# TODO: checksum_tool (⭐ 3/10)

## Status

Completed

## Specification

### Goal

Hash files or standard input with SHA-256, verify checksum manifests, and
optionally traverse directories deterministically.

### Non-goals

- MD5 or other digest algorithms
- Parallel hashing
- Modifying input files

### Inputs and outputs

- Input: file paths, `-` for stdin, `--verify MANIFEST`, or
  `--dir PATH [--extension EXT]`
- Output: `<digest>  <path>` lines or a verification summary

### Errors and limits

- Reject malformed manifest entries and report missing files or digest
  mismatches clearly.
- Sort directory paths before hashing and preserve binary input bytes.

### Acceptance criteria

- [x] SHA-256 hashing works for files and stdin.
- [x] Stable script-friendly output is produced.
- [x] Manifest verification reports success and mismatches.
- [x] Directory traversal supports deterministic ordering and filtering.
- [x] Digest, failure-path, and traversal tests pass.

## Usage

```bash
cargo run --bin checksum_tool
cargo test --bin checksum_tool
```

## Milestones

- [x] Read file paths or stdin data for hashing.
- [x] Implement SHA-256 digest mode.
- [x] Add checksum verification mode against a manifest file.
- [x] Format SHA-256 output as `<digest>  <path>` for scripts and humans.
- [x] Add tests for SHA-256 digest stability.

## Extra

- [x] Add directory traversal with deterministic ordering.

## Tips

- Keep file reading and hash formatting as separate concerns.
- Verification mode should explain mismatches clearly.
- Stable path ordering matters if you add directory support.

## Progress record

- Completed the first slice: SHA-256 hashing from files or stdin with stable
  `<digest>  <path>` output.
- Verification: `cargo test --bin checksum_tool`, strict clippy, and a stdin
  smoke test for `abc` all pass.
- Next slice: add manifest verification with explicit mismatch reporting.
- Manifest verification is now implemented with success and mismatch handling.
- Directory mode now supports recursive traversal, extension filtering, and
  sorted output paths.

## Change record

- Scope: implemented SHA-256 hashing, manifest verification, and deterministic
  directory traversal.
- Tests added: standard digest vectors, output formatting, manifest success and
  mismatch cases, and extension-filtered traversal.
- Commands run: focused `rustfmt`, `cargo check`, `cargo test`, `cargo clippy`,
  and stdin CLI smoke tests.
- Known limitations: SHA-256 is the only supported digest mode.
