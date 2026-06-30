# TODO: edge_cache (⭐ 6/10)

## Usage

```bash
cargo run --bin edge_cache
cargo test --bin edge_cache
```

## Milestones

- [ ] Implement cache key normalization and variant handling.
- [ ] Add TTL-aware object storage with eviction policy.
- [ ] Support conditional revalidation against upstream.
- [ ] Track hit/miss/stale metrics.
- [ ] Add tests for freshness, eviction, and key collisions.

## Extra

- [ ] Add stale-while-revalidate behavior.

## Tips

- Model cache metadata separately from payload bytes.
- Keep revalidation decisions pure so they are easy to unit-test.
