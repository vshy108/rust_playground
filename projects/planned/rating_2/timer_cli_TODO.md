# TODO: timer_cli (⭐ 2/10)

## Usage

```bash
cargo run --bin timer_cli
cargo test --bin timer_cli
```

## Milestones

- [x] Parse durations from CLI arguments.
- [x] Implement countdown display and completion message.
- [x] Add simple stopwatch mode.
- [x] Handle invalid or zero-length durations cleanly.
- [x] Add tests for duration parsing and display formatting helpers.

## Extra

- [x] Add desktop notification hook or sound toggle.

## Tips

- Time parsing and display formatting should be isolated from runtime waiting.
- Use pure helpers for most tests instead of wall-clock timing.
