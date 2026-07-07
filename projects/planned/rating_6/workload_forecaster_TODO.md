# TODO: workload_forecaster (⭐ 6/10)

## Usage

```bash
cargo run --bin workload_forecaster
cargo test --bin workload_forecaster
```

## Milestones

- [ ] Ingest historical workload series and feature signals.
- [ ] Implement baseline trend and seasonality forecasting.
- [ ] Compute confidence intervals for forecast points.
- [ ] Export forecasts for scaling policy consumers.
- [ ] Add tests for trend math and anomaly resilience.

## Extra

- [ ] Add online model recalibration hooks.

## Tips

- Keep forecast models pluggable for side-by-side evaluation.
- Track forecast error over time as a first-class metric.
