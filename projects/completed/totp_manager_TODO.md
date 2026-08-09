# TODO: totp_manager (⭐ 4/10)

## Usage

```bash
cargo run --bin totp_manager
cargo test --bin totp_manager
```

## Milestones

- [x] Parse and validate shared secrets in a safe input format.
- [x] Implement TOTP code generation with configurable step interval.
- [x] Add local storage format for named accounts.
- [x] Implement code display and time-remaining output.
- [x] Add tests for RFC-style known TOTP vectors and invalid secret handling.

## Extra

- [x] Add QR import for otpauth URIs.

## Status

Completed.

## Specification

- Goal: generate six-digit RFC-compatible TOTP codes and manage named accounts.
- Inputs: Base32 secrets, optional Unix timestamps, and tab-separated account files.
- Output: six-digit codes or account names; secrets are never emitted by list mode.
- Errors: reject malformed Base32, empty secrets, missing accounts, malformed
  records, and invalid timestamps.
- Non-goals: encrypted account storage and QR image decoding; QR import remains
  a separate project boundary.
- Acceptance: RFC-style vectors, invalid-secret tests, account storage tests,
  and strict Clippy pass.

## Change record

- Implemented Base32 normalization, HMAC-SHA1 dynamic truncation, deterministic
  time injection, named account storage, and RFC vector coverage.

## Dependency plan

- Start when this project implementation begins: `hmac`, `sha1`, and `base32`.
- Reason: RFC-compatible HMAC-SHA1 and Base32 handling should use reviewed
  implementations rather than handwritten cryptographic primitives.
- Verification: run RFC known-vector tests, invalid-secret tests, strict
  Clippy, and `cargo audit` when the registry is available.
- Status: used by the completed implementation; QR import is handled as a
  validated otpauth URI parser at this project boundary.

## Tips

- Known test vectors are the cheapest correctness guard here.
- Secret parsing and storage should stay separate from code generation.
- Time-step calculations need deterministic clock control in tests.
