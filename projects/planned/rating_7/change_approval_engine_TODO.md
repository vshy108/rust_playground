# TODO: change_approval_engine (⭐ 7/10)

## Usage

```bash
cargo run --bin change_approval_engine
cargo test --bin change_approval_engine
```

## Milestones

- [ ] Define change request model with risk factors.
- [ ] Implement approval policy evaluation workflow.
- [ ] Add multi-step reviewer routing and quorum logic.
- [ ] Persist decision trail with immutable audit entries.
- [ ] Add tests for escalation and timeout behavior.

## Extra

- [ ] Add emergency bypass flow with post-hoc review.

## Tips

- Keep approval policies declarative for easier iteration.
- Separate policy evaluation from notification delivery.
