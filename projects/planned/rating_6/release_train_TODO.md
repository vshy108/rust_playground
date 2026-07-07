# TODO: release_train (⭐ 6/10)

## Usage

```bash
cargo run --bin release_train
cargo test --bin release_train
```

## Milestones

- [ ] Model release train windows, cutoffs, and branch rules.
- [ ] Validate artifact readiness and promotion gates.
- [ ] Automate train assembly with rollback checkpoints.
- [ ] Track release notes and deployment outcomes.
- [ ] Add tests for gate failures and rollback safety.

## Extra

- [ ] Add hotfix lane support outside regular windows.

## Tips

- Keep gate evaluations deterministic so failed trains are reproducible.
- Store rollout checkpoints as immutable snapshots.
