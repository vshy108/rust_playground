# TODO: time_series_db (⭐ 9/10)

## Usage

```bash
cargo run --bin time_series_db
cargo test --bin time_series_db
```

## Milestones

- [ ] Design metric, label, and sample storage layout.
- [ ] Implement append path with per-series indexing.
- [ ] Add retention and compaction for historical data.
- [ ] Implement query engine for range scans and aggregates.
- [ ] Add compression or chunk encoding for sample blocks.
- [ ] Add tests for ingestion order, downsampling, and query correctness.

## Extra

- [ ] Add PromQL-like expression parsing and evaluation.

## Tips

- Separate write-ahead durability from compressed block storage.
- Time ordering assumptions should be explicit and validated.
- Chunk boundaries affect both compression and query latency.
- Build query fixtures from known aggregate expectations.
