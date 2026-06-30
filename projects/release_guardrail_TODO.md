# TODO: release_guardrail (⭐ 7/10)

## Usage

```bash
cargo run --bin release_guardrail
cargo test --bin release_guardrail
```

## Milestones

- [ ] Define guardrail checks for release readiness.
- [ ] Execute health, error-rate, and saturation gates.
- [ ] Block or pause rollout when checks fail.
- [ ] Emit decision evidence for release audits.
- [ ] Add tests for gate ordering and failure behavior.

## Extra

- [ ] Add staged guardrails per deployment phase.

## Tips

- Keep guard checks composable and side-effect free.
- Persist failed gate context for fast diagnosis.
