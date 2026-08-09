# TODO: password_store_cli (⭐ 4/10)

## Usage

```bash
cargo run --bin password_store_cli
cargo test --bin password_store_cli
```

## Milestones

- [x] Store named secrets in a simple local file format.
- [x] Add encrypt/decrypt flow for at-rest protection.
- [x] Implement add, get, list, and delete operations.
- [x] Avoid printing secrets accidentally in listing mode.
- [x] Add tests for storage round-trips and bad-key failures.

## Extra

- [x] Add generated-password support.

## Status

Completed.

## Specification

- Goal: store named secrets with authenticated encryption at rest.
- Inputs: store path, key, and add/get/list/delete/generate commands.
- Output: secret values only for explicit get, names for list, and status messages
  for mutations; list never prints secret contents.
- Errors: reject wrong keys, corrupt stores, malformed records, missing names,
  and file I/O failures.
- Non-goals: key management, password recovery, synchronization, and plaintext
  fallback.
- Acceptance: encrypted round-trip, wrong-key, generated-password, focused test,
  and strict Clippy checks pass.

## Change record

- Implemented AES-GCM authenticated storage, SHA-256 key derivation, Base64 file
  encoding, CRUD commands, safe listing, and generated-password support.

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
