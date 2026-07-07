# TODO: timer_cli (⭐ 2/10)

## Usage

```bash
cargo run --bin timer_cli
cargo test --bin timer_cli
```

## Milestones

- [ ] Parse durations from CLI arguments.
- [ ] Implement countdown display and completion message.
- [ ] Add simple stopwatch mode.
- [ ] Handle invalid or zero-length durations cleanly.
- [ ] Add tests for duration parsing and display formatting helpers.

## Extra

- [ ] Add desktop notification hook or sound toggle.

## Tips

- Time parsing and display formatting should be isolated from runtime waiting.
- Use pure helpers for most tests instead of wall-clock timing.
