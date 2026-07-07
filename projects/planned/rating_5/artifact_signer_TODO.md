# TODO: artifact_signer (⭐ 5/10)

## Usage

```bash
cargo run --bin artifact_signer
cargo test --bin artifact_signer
```

## Milestones

- [ ] Hash files and produce detached signatures or manifests.
- [ ] Verify signatures against stored metadata.
- [ ] Support batch signing for release folders.
- [ ] Report tampering or mismatch details clearly.
- [ ] Add tests for sign and verify flows.

## Extra

- [ ] Add key rotation metadata handling.

## Tips

- Keep hashing and signature storage separate so verification is transparent.
- Test mismatch cases as carefully as happy paths.
