# TODO: capacity_planner (⭐ 6/10)

## Usage

```bash
cargo run --bin capacity_planner
cargo test --bin capacity_planner
```

## Milestones

- [ ] Model workload signals and resource dimensions.
- [ ] Forecast near-term capacity using trend heuristics.
- [ ] Detect saturation risk and recommend scaling actions.
- [ ] Support scenario analysis with what-if inputs.
- [ ] Add tests for forecast math and threshold alerts.

## Extra

- [ ] Add seasonal traffic profile support.

## Tips

- Keep forecasting strategy injectable for easy comparison.
- Separate metric ingestion from recommendation policy.
