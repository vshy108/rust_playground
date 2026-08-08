# TODO: totp_manager (⭐ 4/10)

## Usage

```bash
cargo run --bin totp_manager
cargo test --bin totp_manager
```

## Milestones

- [ ] Parse and validate shared secrets in a safe input format.
- [ ] Implement TOTP code generation with configurable step interval.
- [ ] Add local storage format for named accounts.
- [ ] Implement code display and time-remaining output.
- [ ] Add tests for RFC-style known TOTP vectors and invalid secret handling.

## Extra

- [ ] Add QR import for otpauth URIs.

## Dependency plan

- Start when this project implementation begins: `hmac`, `sha1`, and `base32`.
- Reason: RFC-compatible HMAC-SHA1 and Base32 handling should use reviewed
  implementations rather than handwritten cryptographic primitives.
- Verification: run RFC known-vector tests, invalid-secret tests, strict
  Clippy, and `cargo audit` when the registry is available.
- Status: planned only; these crates are not required by the current scaffold.

## Tips

- Known test vectors are the cheapest correctness guard here.
- Secret parsing and storage should stay separate from code generation.
- Time-step calculations need deterministic clock control in tests.
