# TODO: failover_coordinator (⭐ 8/10)

## Usage

```bash
cargo run --bin failover_coordinator
cargo test --bin failover_coordinator
```

## Milestones

- [ ] Model primary/secondary topology and health policies.
- [ ] Implement failover decision engine with hysteresis.
- [ ] Add state persistence and recovery on restart.
- [ ] Support failback policies with safety checks.
- [ ] Add tests for split-brain and flap scenarios.

## Extra

- [ ] Add region-aware traffic weighting.

## Tips

- Keep decision logic pure and feed it observed health snapshots.
- Persist transitions with reason codes for post-incident analysis.
