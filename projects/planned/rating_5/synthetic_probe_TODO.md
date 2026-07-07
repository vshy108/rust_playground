# TODO: synthetic_probe (⭐ 5/10)

## Usage

```bash
cargo run --bin synthetic_probe
cargo test --bin synthetic_probe
```

## Milestones

- [ ] Define probe scenarios and check assertions.
- [ ] Run probes on intervals with timeout controls.
- [ ] Record latency and outcome metrics.
- [ ] Add alert threshold checks.
- [ ] Add tests for scheduler timing and timeout paths.

## Extra

- [ ] Add geo-distributed probe simulation.

## Tips

- Separate probe execution from result evaluation logic.
- Use deterministic clocks and fake transport in tests.
