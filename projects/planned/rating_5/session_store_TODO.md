# TODO: session_store (⭐ 5/10)

## Usage

```bash
cargo run --bin session_store
cargo test --bin session_store
```

## Milestones

- [ ] Define session schema and expiration semantics.
- [ ] Implement create/get/refresh/delete operations.
- [ ] Add TTL sweep or lazy expiration strategy.
- [ ] Support secure session token generation.
- [ ] Add tests for expiry and concurrent updates.

## Extra

- [ ] Add session activity timeline tracking.

## Tips

- Keep token identity and session payload separate.
- Inject the clock so expiration logic is deterministic in tests.
