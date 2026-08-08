# TODO: checksum_tool (⭐ 3/10)

## Usage

```bash
cargo run --bin checksum_tool
cargo test --bin checksum_tool
```

## Milestones

- [x] Read file paths or stdin data for hashing.
- [x] Implement SHA-256 digest mode.
- [ ] Add checksum verification mode against a manifest file.
- [x] Format SHA-256 output as `<digest>  <path>` for scripts and humans.
- [x] Add tests for SHA-256 digest stability.

## Extra

- [ ] Add directory traversal with deterministic ordering.

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
