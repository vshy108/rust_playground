# TODO: incident_router (⭐ 6/10)

## Usage

```bash
cargo run --bin incident_router
cargo test --bin incident_router
```

## Milestones

- [ ] Define incident envelope, source metadata, and severity model.
- [ ] Route incidents to teams using rule-based matching.
- [ ] Add escalation paths and on-call fallback behavior.
- [ ] Add rate-limiting or dedup for noisy incidents.
- [ ] Add tests for routing precedence and escalation timing.

## Extra

- [ ] Add quiet-window support for non-critical incidents.

## Tips

- Keep routing logic deterministic so incidents are reproducible in tests.
- Separate classification from delivery to keep responsibilities clear.
