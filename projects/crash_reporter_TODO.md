# TODO: crash_reporter (⭐ 7/10)

## Usage

```bash
cargo run --bin crash_reporter
cargo test --bin crash_reporter
```

## Milestones

- [ ] Implement crash event intake schema with stack traces and release metadata.
- [ ] Add symbol or source map lookup abstraction.
- [ ] Implement event fingerprinting and issue grouping rules.
- [ ] Add issue state transitions, regression detection, and release tracking.
- [ ] Implement retention policy and searchable event indexing.
- [ ] Add tests for grouping stability, regression reopening, and malformed payload handling.

## Extra

- [ ] Add alert routing for new high-severity regressions.

## Tips

- Grouping rules define product usefulness; keep them deterministic.
- Event ingestion and issue lifecycle should be independent modules.
- Release metadata makes regressions understandable, not just countable.
- Malformed stack traces are normal production input, not edge cases.
