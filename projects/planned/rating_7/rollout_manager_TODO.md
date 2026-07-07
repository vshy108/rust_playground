# TODO: rollout_manager (⭐ 7/10)

## Usage

```bash
cargo run --bin rollout_manager
cargo test --bin rollout_manager
```

## Milestones

- [ ] Define rollout plans with staged percentages and guardrails.
- [ ] Implement plan execution with pause/resume controls.
- [ ] Add health check integration for safe progression.
- [ ] Add rollback plans and blast-radius controls.
- [ ] Add tests for progression logic and failure handling.

## Extra

- [ ] Add calendar-aware rollout windows.

## Tips

- Separate plan definition from runtime execution state.
- Persist checkpoints often so restarts do not lose rollout progress.
