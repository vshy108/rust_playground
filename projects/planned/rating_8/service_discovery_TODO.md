# TODO: service_discovery (⭐ 8/10)

## Usage

```bash
cargo run --bin service_discovery
cargo test --bin service_discovery
```

## Milestones

- [ ] Design service registration records and health state model.
- [ ] Implement register, deregister, and heartbeat flows.
- [ ] Add query API for exact match and tag-based discovery.
- [ ] Implement TTL expiry and unhealthy instance eviction.
- [ ] Add watch/subscribe support for discovery changes.
- [ ] Add tests for expiry timing, concurrent updates, and lookup correctness.

## Extra

- [ ] Add multi-zone routing or weighted selection.

## Tips

- Keep the registry state model explicit and easy to snapshot.
- Expiry logic should be deterministic in tests; use injectable clocks where practical.
- Treat health and presence as related but separate concerns.
- Watch delivery order matters when clients cache results.
