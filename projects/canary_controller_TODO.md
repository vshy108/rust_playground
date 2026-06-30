# TODO: canary_controller (⭐ 8/10)

## Usage

```bash
cargo run --bin canary_controller
cargo test --bin canary_controller
```

## Milestones

- [ ] Model rollout stages and promotion criteria.
- [ ] Collect health signals and compute safety verdicts.
- [ ] Implement automated promote/rollback transitions.
- [ ] Persist rollout history for auditability.
- [ ] Add tests for state transitions and rollback triggers.

## Extra

- [ ] Add multi-service coordinated canary support.

## Tips

- Treat rollout logic as a state machine first, then add adapters.
- Keep signal ingestion asynchronous but decision logic deterministic.
