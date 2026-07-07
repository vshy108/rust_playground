# TODO: metrics_collector (⭐ 6/10)


## Usage

```bash
cargo run --bin metrics_collector
cargo test --bin metrics_collector
```

## 1. Metric types

- [ ] Define a `MetricEvent` enum: `Counter { name, value: f64 }`, `Gauge { name, value: f64 }`,
  `Histogram { name, value: f64 }`.
- [ ] Define an `Aggregation` struct: count, sum, min, max per metric name.

Acceptance check: structs compile; a `MetricEvent` can be pattern-matched.

## 2. Aggregator loop

- [ ] Spawn an aggregator task that owns a `HashMap<String, Aggregation>`.
- [ ] Receive `MetricEvent` values from an mpsc channel; update the aggregation on each event.

Acceptance check: sending 5 counter events results in a sum of 5 in the aggregation.

## 3. Query interface

- [ ] Add a `query(name: &str) -> Option<Aggregation>` snapshot read.
- [ ] Snapshot must not hold the lock while doing other work (lock, clone, drop).

Acceptance check: query returns the correct aggregation after a burst of events.

## 4. Tests

- [ ] Counter aggregation accumulates correctly.
- [ ] Gauge last-write-wins (or tracks min/max).
- [ ] Histogram min/max/mean over a known series.

## Extra: Prometheus endpoint

- [ ] Add an axum route `GET /metrics` that serialises all aggregations in the
  Prometheus text exposition format (`# TYPE`, `metric_name value timestamp`).

## Tips

- Start with a deterministic local fixture path before external integration.
- Add bounded concurrency controls early to prevent overload and flakiness.
- Separate collection from aggregation/output so each can be tested in isolation.
- Add backoff and retry policy tests for transient failures.
- Measure throughput and tail latency on representative input sizes.

## Learn Notes

- channels — `std::sync::mpsc` (or `tokio::sync::mpsc`) decouples metric producers from the aggregator; producers send `MetricEvent` values; the aggregator owns the receiver
- aggregation — the aggregator loop accumulates counters and histograms in a HashMap; a query interface reads snapshots without blocking producers

## Extra

- Prometheus endpoint — expose `/metrics` in the text-based Prometheus exposition format

