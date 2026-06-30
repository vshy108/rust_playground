# TODO: config_deployer (⭐ 6/10)

## Usage

```bash
cargo run --bin config_deployer
cargo test --bin config_deployer
```

## Milestones

- [ ] Load desired config from files or templates.
- [ ] Compare desired state to target machine or directory state.
- [ ] Apply changes with a clear plan step.
- [ ] Add rollback or backup before overwrite.
- [ ] Add tests for diff and apply behavior.

## Extra

- [ ] Add remote transport support over SSH.

## Tips

- Make dry-run output a first-class feature from the start.
- Diff planning should be independent from how changes are applied.
