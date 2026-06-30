# TODO: webhook_dispatcher (⭐ 6/10)

## Usage

```bash
cargo run --bin webhook_dispatcher
cargo test --bin webhook_dispatcher
```

## Milestones

- [ ] Queue outbound webhook events with payload metadata.
- [ ] Deliver events to configured endpoints.
- [ ] Track attempts, retries, and final outcomes.
- [ ] Add signature headers or shared-secret verification.
- [ ] Add tests for retry and delivery bookkeeping.

## Extra

- [ ] Add per-endpoint concurrency and backoff policies.

## Tips

- Separate event storage from HTTP delivery so retries stay deterministic.
- Delivery history is part of the product, not just debug output.
