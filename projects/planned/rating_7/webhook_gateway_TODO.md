# TODO: webhook_gateway (⭐ 7/10)

## Usage

```bash
cargo run --bin webhook_gateway
cargo test --bin webhook_gateway
```

## Milestones

- [ ] Define webhook subscription and delivery models.
- [ ] Implement outbound delivery with retries and backoff.
- [ ] Add request signing and signature verification helpers.
- [ ] Implement dead-letter handling for exhausted deliveries.
- [ ] Add observability for attempts, latency, and failure causes.
- [ ] Add tests for retry timing, signature mismatches, and idempotent redelivery.

## Extra

- [ ] Add rate limits and per-destination circuit breaking.

## Tips

- Store delivery attempts separately from subscription metadata.
- Sign canonical payloads, not reconstructed request bodies.
- Retry logic should be deterministic in tests.
- Dead-letter visibility matters more than throughput early on.
