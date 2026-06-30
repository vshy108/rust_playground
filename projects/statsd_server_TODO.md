# TODO: statsd_server (⭐ 7/10)

## Usage

```bash
cargo run --bin statsd_server
cargo test --bin statsd_server
```

## Milestones

- [ ] Implement StatsD line parsing for counters, gauges, timers, and sets.
- [ ] Add in-memory aggregation windows and flush intervals.
- [ ] Implement sample-rate handling and metric key normalization.
- [ ] Add exporter abstraction for logs, Prometheus, or custom sinks.
- [ ] Implement cardinality controls and error accounting.
- [ ] Add tests for parser correctness, sampling math, and flush semantics.

## Extra

- [ ] Add tag support compatible with a modern StatsD dialect.

## Tips

- Parsing, aggregation, and exporting should be separate stages.
- Flush interval behavior needs fake time in tests.
- Sample-rate handling is correctness-critical for counters and timers.
- Cardinality controls should be visible, not silent drops.
