# TODO: stream_processor (⭐ 9/10)

## Usage

```bash
cargo run --bin stream_processor
cargo test --bin stream_processor
```

## Milestones

- [ ] Define event schema and operator graph model.
- [ ] Implement source -> transform -> sink pipeline execution.
- [ ] Add keyed state and windowed aggregations.
- [ ] Implement watermark or event-time progress tracking.
- [ ] Add checkpointing and replay-safe recovery model.
- [ ] Add tests for ordering, late events, and state restoration.

## Extra

- [ ] Add SQL-like query layer over the operator graph.

## Tips

- State isolation per operator is easier to reason about than global mutable state.
- Event-time handling deserves explicit tests; intuitive implementations drift fast.
- Build replay fixtures before optimizing throughput.
- Checkpoint format stability matters if you later add upgrades.
