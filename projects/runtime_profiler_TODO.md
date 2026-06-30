# TODO: runtime_profiler (⭐ 5/10)

## Usage

```bash
cargo run --bin runtime_profiler
cargo test --bin runtime_profiler
```

## Milestones

- [ ] Sample runtime counters and timing spans.
- [ ] Attribute cost to functions or task groups.
- [ ] Produce top-hotspots report output.
- [ ] Add threshold alerts for regressions.
- [ ] Add tests for aggregation and percentile math.

## Extra

- [ ] Add flamegraph export format.

## Tips

- Keep sampling overhead visible and bounded.
- Separate collection from reporting so both stay testable.
