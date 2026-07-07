# TODO: maintenance_window_manager (⭐ 5/10)


## Usage

```bash
cargo run --bin maintenance_window_manager
cargo test --bin maintenance_window_manager
```

## Milestones

- [ ] Model windows, blackout periods, and affected scopes.
- [ ] Validate schedule overlaps and policy conflicts.
- [ ] Add approval flow for high-impact windows.
- [ ] Emit reminders and execution checkpoints.
- [ ] Add tests for overlap, timezone, and recurrence logic.

## Extra

- [ ] Add calendar export integration.

## Tips

- Centralize time math to avoid timezone edge bugs.
- Keep policy validation pure and deterministic.
