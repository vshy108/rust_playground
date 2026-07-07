# TODO: secrets_manager (⭐ 8/10)

## Usage

```bash
cargo run --bin secrets_manager
cargo test --bin secrets_manager
```

## Milestones

- [ ] Implement secret storage abstraction with encryption-at-rest boundaries.
- [ ] Add key hierarchy, rotation metadata, and versioned secret values.
- [ ] Implement CRUD APIs with least-privilege policy checks.
- [ ] Add lease or TTL semantics for dynamic credentials.
- [ ] Implement audit logging for secret reads and writes.
- [ ] Add tests for access control, rotation behavior, and encrypted persistence.

## Extra

- [ ] Add pluggable KMS backend support.

## Tips

- Keep plaintext handling narrowly scoped and easy to audit.
- Secret versioning and key rotation are different lifecycle concerns.
- Policy evaluation should be deterministic and independently testable.
- Audit trails are part of correctness, not optional observability.
