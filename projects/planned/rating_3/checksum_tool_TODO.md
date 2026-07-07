# TODO: checksum_tool (⭐ 3/10)

## Usage

```bash
cargo run --bin checksum_tool
cargo test --bin checksum_tool
```

## Milestones

- [ ] Read file paths or stdin data for hashing.
- [ ] Implement one or two digest modes such as SHA-256 and MD5.
- [ ] Add checksum verification mode against a manifest file.
- [ ] Format output for both human and script use.
- [ ] Add tests for digest stability and verify-mode failures.

## Extra

- [ ] Add directory traversal with deterministic ordering.

## Tips

- Keep file reading and hash formatting as separate concerns.
- Verification mode should explain mismatches clearly.
- Stable path ordering matters if you add directory support.
