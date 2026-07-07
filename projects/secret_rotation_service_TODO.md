# TODO: secret_rotation_service (⭐ 7/10)


## Usage

```bash
cargo run --bin secret_rotation_service
cargo test --bin secret_rotation_service
```

## Milestones

- [ ] Model secret versions and rotation policies.
- [ ] Schedule rotations and coordinate rollout steps.
- [ ] Add grace periods and rollback for failed rotations.
- [ ] Track rotation history and compliance status.
- [ ] Add tests for version promotion and rollback behavior.

## Extra

- [ ] Add emergency rotate-now workflow.

## Tips

- Never overwrite secret versions; create immutable generations.
- Separate key material adapters from policy orchestration.
