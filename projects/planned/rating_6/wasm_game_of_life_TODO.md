# TODO: wasm_game_of_life (⭐ 6/10)

## Usage

```bash
cargo run --bin wasm_game_of_life
cargo test --bin wasm_game_of_life
```

## Milestones

- [ ] Model the universe grid and update rules in pure Rust.
- [ ] Expose a minimal wasm-friendly API for initialization and ticking.
- [ ] Add JS/HTML glue for rendering and interaction.
- [ ] Add efficient cell diff or framebuffer updates for redraws.
- [ ] Add profiling hooks for wasm size and frame timing.
- [ ] Add tests for rule correctness and edge wrapping behavior.

## Extra

- [ ] Add editable patterns and seeded random worlds.

## Tips

- Start with a deterministic local fixture path before external integration.
- Add bounded concurrency controls early to prevent overload and flakiness.
- Separate collection from aggregation/output so each can be tested in isolation.
- Add backoff and retry policy tests for transient failures.
- Measure throughput and tail latency on representative input sizes.
