# TODO: password_store_cli (⭐ 4/10)

## Usage

```bash
cargo run --bin password_store_cli
cargo test --bin password_store_cli
```

## Milestones

- [ ] Store named secrets in a simple local file format.
- [ ] Add encrypt/decrypt flow for at-rest protection.
- [ ] Implement add, get, list, and delete operations.
- [ ] Avoid printing secrets accidentally in listing mode.
- [ ] Add tests for storage round-trips and bad-key failures.

## Extra

- [ ] Add generated-password support.

## Dependency plan

- Start when this project implementation begins: `aes-gcm`, `sha2`, and
  `base64`.
- Reason: authenticated encryption, deterministic key derivation input hashing,
  and safe text storage encoding are not responsibilities to implement locally.
- Verification: run focused tests, strict Clippy, and `cargo audit` when the
  registry is available.
- Security rule: do not substitute homemade encryption or plaintext storage.

## Tips

- Keep crypto boundaries narrow and explicit.
- Metadata listing and secret retrieval should be separate commands.
- Round-trip tests catch most storage issues early.
