# TODO: blast_radius_analyzer (⭐ 7/10)

## Usage

```bash
cargo run --bin blast_radius_analyzer
cargo test --bin blast_radius_analyzer
```

## Milestones

- [ ] Model service/resource dependency graph inputs.
- [ ] Compute impact sets for failure or change scenarios.
- [ ] Add criticality scoring and confidence levels.
- [ ] Generate operator-friendly impact reports.
- [ ] Add tests for traversal correctness and score stability.

## Extra

- [ ] Add what-if simulation over historical incidents.

## Tips

- Keep graph normalization separate from impact algorithms.
- Cache repeated subgraph computations for scale.
