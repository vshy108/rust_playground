# TODO: traffic_replayer (⭐ 7/10)

## Usage

```bash
cargo run --bin traffic_replayer
cargo test --bin traffic_replayer
```

## Milestones

- [ ] Model captured request streams and timing metadata.
- [ ] Replay traffic with rate and concurrency controls.
- [ ] Add endpoint mapping and payload mutation hooks.
- [ ] Measure replay latency and response differences.
- [ ] Add tests for schedule fidelity and replay safety.

## Extra

- [ ] Add deterministic seed mode for reproducible runs.

## Tips

- Keep capture format versioned to avoid parser drift.
- Isolate transport adapters from replay scheduling logic.
