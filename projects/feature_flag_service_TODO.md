# TODO: feature_flag_service (⭐ 6/10)

## Usage

```bash
cargo run --bin feature_flag_service
cargo test --bin feature_flag_service
```

## Milestones

- [ ] Model projects, environments, and flag rules.
- [ ] Serve flag evaluations over a small HTTP API.
- [ ] Add percentage rollout and targeting basics.
- [ ] Support local persistence for flag definitions.
- [ ] Add tests for evaluation edge cases and rollout math.

## Extra

- [ ] Add audit history for flag changes.

## Tips

- Separate rule evaluation from transport so the logic stays testable.
- Percentage rollouts are easier to reason about with deterministic hashing.
