# TODO: event_sourcing_db (⭐ 9/10)

## Usage

```bash
cargo run --bin event_sourcing_db
cargo test --bin event_sourcing_db
```

## Milestones

- [ ] Implement append-only event log and stream identifiers.
- [ ] Add optimistic concurrency checks for stream writes.
- [ ] Implement snapshotting and state rehydration.
- [ ] Add projection engine for read models.
- [ ] Implement retention or archival policy for cold streams.
- [ ] Add tests for version conflicts, projection rebuilds, and snapshot recovery.

## Extra

- [ ] Add subscription API for live event consumers.

## Tips

- Append-only guarantees should be explicit and test-covered.
- Write path concurrency rules define the whole model; document them early.
- Projections are disposable; keep them rebuildable from the event log.
- Snapshot format stability matters if you evolve event schemas.
