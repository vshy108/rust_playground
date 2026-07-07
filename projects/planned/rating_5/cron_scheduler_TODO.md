# TODO: cron_scheduler (⭐ 5/10)

## Usage

```bash
cargo run --bin cron_scheduler
cargo test --bin cron_scheduler
```

## Milestones

- [ ] Parse a minimal cron-like schedule expression.
- [ ] Compute the next execution time from a base timestamp.
- [ ] Run shell commands or internal tasks on schedule.
- [ ] Add graceful shutdown and missed-run handling.
- [ ] Add tests for schedule parsing and boundary times.

## Extra

- [ ] Add persistent job definitions in a config file.

## Tips

- Start with minute-level scheduling before widening the cron grammar.
- Keep the next-run calculation pure and heavily tested.
