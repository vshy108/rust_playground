# TODO: dependency_risk_heatmap (⭐ 6/10)


## Usage

```bash
cargo run --bin dependency_risk_heatmap
cargo test --bin dependency_risk_heatmap
```

## Milestones

- [ ] Build dependency graph with ownership mapping.
- [ ] Score nodes by vulnerability and staleness risk.
- [ ] Render heatmap-ready output grouped by service/team.
- [ ] Highlight high-centrality risky dependencies.
- [ ] Add tests for scoring consistency and grouping.

## Extra

- [ ] Add change-over-time risk trend views.

## Tips

- Keep scoring weights configurable for policy tuning.
- Separate graph metrics from presentation formatting.
