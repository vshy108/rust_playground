# TODO: cache_invalidator (⭐ 6/10)

## Usage

```bash
cargo run --bin cache_invalidator
cargo test --bin cache_invalidator
```

## Milestones

- [ ] Model invalidation events and target selectors.
- [ ] Implement rule-based and explicit key invalidation.
- [ ] Add batched propagation and retry behavior.
- [ ] Add idempotency and de-dup safeguards.
- [ ] Add tests for fan-out ordering and failure recovery.

## Extra

- [ ] Add soft purge vs hard purge modes.

## Tips

- Design invalidation IDs early to support retries safely.
- Keep propagation adapters separate from decision rules.
