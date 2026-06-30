# TODO: rate_limiter (⭐ 7/10)

## Usage

```bash
cargo run --bin rate_limiter
cargo test --bin rate_limiter
```

## Milestones

- [ ] Implement token bucket limiter.
- [ ] Implement sliding window limiter.
- [ ] Expose HTTP middleware example usage.
- [ ] Add per-key limiter map with TTL eviction.
- [ ] Add metrics (allowed, denied, wait time).
- [ ] Add property-style tests for edge windows.

## Extra

- [ ] Add distributed mode with Redis backend.

## Tips

- Implement one milestone at a time and keep each slice testable.
- Add a failing test first, then implement the smallest behavior to pass.
- Keep CLI/API surface stable while iterating internals.
- Validate both happy path and error path for every milestone.
- Run focused tests before broad checks.
