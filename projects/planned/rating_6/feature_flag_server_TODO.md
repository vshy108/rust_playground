# TODO: feature_flag_server (⭐ 6/10)

## Usage

```bash
cargo run --bin feature_flag_server
cargo test --bin feature_flag_server
```

## Milestones

- [ ] Model flags, environments, and targeting rules.
- [ ] Implement evaluation API for user/context requests.
- [ ] Add percentage rollout and segment matching support.
- [ ] Implement audit log or change history tracking.
- [ ] Add flag cache invalidation and consistency rules.
- [ ] Add tests for deterministic rollout bucketing and rule precedence.

## Extra

- [ ] Add experiment metrics hooks and variant assignment.

## Tips

- Rollout bucketing must be stable across runs; test hashing explicitly.
- Separate admin mutation paths from hot evaluation paths.
- Explainability helps debugging targeting mistakes.
- A small rule language beats ad hoc branching logic.
