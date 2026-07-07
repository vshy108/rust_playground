# TODO: artifact_proxy (⭐ 8/10)

## Usage

```bash
cargo run --bin artifact_proxy
cargo test --bin artifact_proxy
```

## Milestones

- [ ] Implement upstream fetch abstraction for package or artifact sources.
- [ ] Add local cache with immutable blob addressing.
- [ ] Implement metadata rewriting or pass-through rules for proxied registries.
- [ ] Add auth, namespace policy, and air-gap mode behavior.
- [ ] Implement cache eviction and integrity verification.
- [ ] Add tests for cache hits, stale metadata refresh, and integrity mismatch handling.

## Extra

- [ ] Add multi-format support beyond one package ecosystem.

## Tips

- Metadata and blob caching have different invalidation semantics.
- Integrity verification should happen before serving cached artifacts.
- Air-gap mode changes failure behavior; treat it as a first-class mode.
- Start with one ecosystem contract before generalizing to many.
