# TODO: config_diff_auditor (⭐ 6/10)

## Usage

```bash
cargo run --bin config_diff_auditor
cargo test --bin config_diff_auditor
```

## Milestones

- [ ] Parse baseline and candidate configuration states.
- [ ] Compute semantic diffs with ignore rules.
- [ ] Classify risky changes with policy checks.
- [ ] Emit machine-readable and human-readable reports.
- [ ] Add tests for nested diff behavior and risk detection.

## Extra

- [ ] Add drift timeline history view.

## Tips

- Parse into typed models before diffing for stability.
- Keep policy checks independent from diff algorithm details.
