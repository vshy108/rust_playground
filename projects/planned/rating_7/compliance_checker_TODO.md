# TODO: compliance_checker (⭐ 7/10)

## Usage

```bash
cargo run --bin compliance_checker
cargo test --bin compliance_checker
```

## Milestones

- [ ] Define policy rules and evidence collection model.
- [ ] Evaluate resources against policy checks.
- [ ] Add waiver/exception support with expiration.
- [ ] Produce compliance reports with severity levels.
- [ ] Add tests for rule evaluation and report accuracy.

## Extra

- [ ] Add policy bundle versioning.

## Tips

- Keep policy parsing independent from execution context.
- Return structured evidence to simplify audit reviews.
