# TODO: config_manager (⭐ 7/10)

## Usage

```bash
cargo run --bin config_manager
cargo test --bin config_manager
```

## Milestones

- [ ] Design declarative config format for files, commands, and packages.
- [ ] Implement desired-vs-actual state diffing.
- [ ] Add apply engine with ordered reconciliation steps.
- [ ] Implement rollback-safe failure reporting or partial-apply summaries.
- [ ] Add dry-run and plan output modes.
- [ ] Add tests for idempotent apply, drift detection, and failure handling.

## Extra

- [ ] Add remote target support over SSH.

## Tips

- Idempotency is the central property; test it from the start.
- Keep fact gathering separate from mutation steps.
- Dry-run output should reuse the same diff model as apply.
- Record why a change happened, not just that it happened.
