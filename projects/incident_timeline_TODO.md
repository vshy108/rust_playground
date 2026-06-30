# TODO: incident_timeline (⭐ 5/10)

## Usage

```bash
cargo run --bin incident_timeline
cargo test --bin incident_timeline
```

## Milestones

- [ ] Define timeline event schema and ordering rules.
- [ ] Ingest incident events from multiple sources.
- [ ] Merge and sort events with stable tie-breakers.
- [ ] Render text and JSON timeline views.
- [ ] Add tests for ordering, dedup, and rendering.

## Extra

- [ ] Add causal-link hints between events.

## Tips

- Normalize timestamps and source IDs early in the pipeline.
- Keep timeline rendering separate from event ingestion.
