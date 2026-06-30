# TODO: authz_drift_detector (⭐ 8/10)

## Usage

```bash
cargo run --bin authz_drift_detector
cargo test --bin authz_drift_detector
```

## Milestones

- [ ] Model desired and observed authorization states.
- [ ] Detect privilege drift and missing controls.
- [ ] Classify drift severity by resource sensitivity.
- [ ] Generate remediation plans with ownership.
- [ ] Add tests for diffing and severity classification.

## Extra

- [ ] Add continuous drift watch mode.

## Tips

- Compare normalized policy graphs, not raw text.
- Keep detection deterministic for audit reproducibility.
