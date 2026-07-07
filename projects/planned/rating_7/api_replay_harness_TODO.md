# TODO: api_replay_harness (⭐ 7/10)

## Usage

```bash
cargo run --bin api_replay_harness
cargo test --bin api_replay_harness
```

## Milestones

- [ ] Model captured API sessions and replay constraints.
- [ ] Reconstruct request order with dependency edges.
- [ ] Compare replay responses against expected baselines.
- [ ] Add masking for sensitive fields in reports.
- [ ] Add tests for deterministic ordering and diff output.

## Extra

- [ ] Add selective endpoint replay filters.

## Tips

- Treat replay datasets as immutable test fixtures.
- Separate execution transport from comparison logic.
