# TODO: schema_migration_planner (⭐ 8/10)


## Usage

```bash
cargo run --bin schema_migration_planner
cargo test --bin schema_migration_planner
```

## Milestones

- [ ] Model schema versions and migration operations.
- [ ] Build forward/backward compatibility checks.
- [ ] Generate ordered migration plans with safety gates.
- [ ] Support dry-run diff output and rollback planning.
- [ ] Add tests for dependency ordering and failure handling.

## Extra

- [ ] Add cross-service migration coordination mode.

## Tips

- Treat migration steps as immutable units with explicit preconditions.
- Keep compatibility checks independent from execution adapters.
