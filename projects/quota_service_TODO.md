# TODO: quota_service (⭐ 5/10)

## Usage

```bash
cargo run --bin quota_service
cargo test --bin quota_service
```

## Milestones

- [ ] Model quota dimensions (tenant, endpoint, window).
- [ ] Implement consume/check operations with counters.
- [ ] Support fixed-window and sliding-window strategies.
- [ ] Add reset and admin inspection APIs.
- [ ] Add tests for limit boundaries and window rollover.

## Extra

- [ ] Add burst credits and refill pacing.

## Tips

- Keep time source injectable for deterministic tests.
- Return structured quota decisions so callers can expose useful errors.
