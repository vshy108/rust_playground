# TODO: delivery_slo_guard (⭐ 7/10)

## Usage

```bash
cargo run --bin delivery_slo_guard
cargo test --bin delivery_slo_guard
```

## Milestones

- [ ] Define delivery SLOs and burn-rate thresholds.
- [ ] Ingest delivery latency and failure events.
- [ ] Compute windowed compliance and regression signals.
- [ ] Trigger mitigation recommendations on risk breach.
- [ ] Add tests for burn-rate and window math.

## Extra

- [ ] Add per-customer SLO slicing.

## Tips

- Keep time-window aggregation deterministic in tests.
- Separate SLO policy definitions from metric ingestion.
