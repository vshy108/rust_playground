# TODO: metrics_dashboard (⭐ 6/10)

## Usage

```bash
cargo run --bin metrics_dashboard
cargo test --bin metrics_dashboard
```

## Milestones

- [ ] Ingest metrics snapshots from a local file or stream.
- [ ] Aggregate series by name and time window.
- [ ] Render a simple terminal or web dashboard.
- [ ] Add threshold or anomaly highlighting.
- [ ] Add tests for aggregation and rendering logic.

## Extra

- [ ] Add live refresh mode with rolling windows.

## Tips

- Normalize input into one internal metric shape before rendering.
- Dashboard layout code should not own aggregation rules.
