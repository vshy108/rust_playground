# TODO: threat_feed_aggregator (⭐ 8/10)

## Usage

```bash
cargo run --bin threat_feed_aggregator
cargo test --bin threat_feed_aggregator
```

## Milestones

- [ ] Ingest indicators from multiple feed formats.
- [ ] Normalize and deduplicate indicator records.
- [ ] Add confidence scoring and source weighting.
- [ ] Expose lookup API for downstream consumers.
- [ ] Add tests for feed parsing and merge correctness.

## Extra

- [ ] Add STIX/TAXII adaptor support.

## Tips

- Normalize early so all downstream logic uses one canonical model.
- Track source provenance per indicator for explainability.
