# TODO: latency_budget_planner (⭐ 6/10)

## Usage

```bash
cargo run --bin latency_budget_planner
cargo test --bin latency_budget_planner
```

## Milestones

- [ ] Define end-to-end latency budget contracts.
- [ ] Allocate per-hop latency envelopes.
- [ ] Detect budget overruns and bottleneck services.
- [ ] Recommend reallocation or optimization actions.
- [ ] Add tests for budget math and bottleneck detection.

## Extra

- [ ] Add percentile-based budget planning.

## Tips

- Treat budget propagation as a first-class data flow.
- Keep percentile calculations reusable across modules.
