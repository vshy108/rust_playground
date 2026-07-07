# TODO: incident_simulator (⭐ 6/10)

## Usage

```bash
cargo run --bin incident_simulator
cargo test --bin incident_simulator
```

## Milestones

- [ ] Define incident scenario templates and trigger events.
- [ ] Simulate alert bursts, delays, and recovery phases.
- [ ] Add team response timeline generation.
- [ ] Emit metrics for detection and mitigation latency.
- [ ] Add tests for deterministic scenario execution.

## Extra

- [ ] Add Monte Carlo mode for resilience scoring.

## Tips

- Keep scenario generation seedable for reproducibility.
- Separate simulation state from reporting layers.
