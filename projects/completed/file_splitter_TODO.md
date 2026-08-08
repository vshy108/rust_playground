# TODO: file_splitter (⭐ 3/10)

## Status

Completed

## Specification

### Goal

Split files by bytes or lines, optionally gzip parts, generate manifests, and
reassemble parts deterministically.

### Non-goals

- Parallel splitting
- Compression formats other than the dependency-free stored gzip mode
- Automatic deletion of source or part files

### Inputs and outputs

- Input: `--lines N` or `--bytes N`, source file, optional `--manifest` and
  `--gzip`; `--join MANIFEST OUTPUT` for reassembly
- Output: deterministic part names, optional manifest, and reassembled output

### Errors and limits

- Reject zero/invalid sizes and malformed gzip data.
- Preserve exact bytes and validate gzip checksums during reassembly.

### Acceptance criteria

- [x] Line and byte splitting handle exact boundaries.
- [x] Part names and manifests are deterministic.
- [x] Reassembly preserves original bytes.
- [x] Optional gzip parts round-trip with checksum validation.
- [x] Focused tests and clippy pass.

## Usage

```bash
cargo run --bin file_splitter
cargo test --bin file_splitter
```

## Milestones

- [x] Split a file by line count or byte size.
- [x] Generate deterministic part names.
- [x] Add reassembly and manifest metadata mode.
- [x] Handle tiny files and exact-boundary splits correctly.
- [x] Add tests for split sizes and part naming.

## Extra

- [x] Add compression option for output parts.

## Tips

- Naming rules should be stable and easy to test.
- Byte-based and line-based split logic should stay separate.

## Progress record

- Implemented the line/byte splitting core and deterministic part naming.
- Verification: focused tests and strict clippy are the next gate.
- Remaining: reassembly/manifest metadata and optional compression.
- Manifest generation and `--join` reassembly are now implemented.
- Dependency-free gzip stored-block output and checksum-checked reassembly are
  now implemented.

## Change record

- Scope: completed splitting, deterministic manifests, reassembly, and gzip
  stored-block output.
- Tests added: exact boundaries, naming, and gzip round-trip coverage.
- Commands run: focused `rustfmt`, `cargo check`, `cargo test`, and `cargo clippy`.
