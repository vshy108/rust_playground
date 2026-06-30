# TODO: policy_simulator (⭐ 6/10)

## Usage

```bash
cargo run --bin policy_simulator
cargo test --bin policy_simulator
```

## Milestones

- [ ] Parse policy definitions and evaluation inputs.
- [ ] Execute policy decisions in dry-run mode.
- [ ] Emit explain traces for allow/deny outcomes.
- [ ] Support batch simulations for change previews.
- [ ] Add tests for decision correctness and traces.

## Extra

- [ ] Add policy diff view between two revisions.

## Tips

- Separate policy parsing from evaluation to simplify tests.
- Ensure traces include both matched and skipped rules.
