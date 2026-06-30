# TODO: power_monitor (⭐ 6/10)

## Usage

```bash
cargo run --bin power_monitor
cargo test --bin power_monitor
```

## Milestones

- [ ] Implement host power metric collection abstraction.
- [ ] Add periodic sampling and rolling window aggregation.
- [ ] Implement process or subsystem attribution model where available.
- [ ] Add threshold alerts and summary reporting.
- [ ] Implement export format for historical samples.
- [ ] Add tests for aggregation math, threshold logic, and missing-metric handling.

## Extra

- [ ] Add a TUI dashboard with per-component trend charts.

## Tips

- Sampling and presentation should be independent layers.
- Missing metrics are normal on some platforms; model them explicitly.
- Aggregation windows need deterministic time handling in tests.
- Attribution support can begin as a best-effort optional feature.
