# TODO: tracing_backend (⭐ 8/10)

## Usage

```bash
cargo run --bin tracing_backend
cargo test --bin tracing_backend
```

## Milestones

- [ ] Define span, event, and trace storage model.
- [ ] Implement ingest API for trace batches.
- [ ] Add trace assembly from spans with parent-child relationships.
- [ ] Implement search/filtering by service, time range, and trace id.
- [ ] Add retention policy and storage compaction.
- [ ] Add tests for out-of-order spans, missing parents, and query correctness.

## Extra

- [ ] Add latency heatmaps or critical path analysis.

## Tips

- Ingestion order will often differ from logical span order; design for it.
- Separate write path from query indexes early.
- Trace assembly should tolerate partial data without panicking.
- Time-range queries are a natural place to add fixtures with expected ordering.
