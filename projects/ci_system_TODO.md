# TODO: ci_system (⭐ 7/10)

## Usage

```bash
cargo run --bin ci_system
cargo test --bin ci_system
```

## Milestones

- [ ] Parse a pipeline definition with jobs, steps, and dependencies.
- [ ] Implement a local worker that executes commands with captured logs.
- [ ] Add job state transitions and failure propagation across the DAG.
- [ ] Add artifact passing or workspace persistence between steps.
- [ ] Add retry, timeout, and cancellation behavior.
- [ ] Add tests for DAG scheduling, retries, and log collection.

## Extra

- [ ] Add remote worker protocol and queued execution.

## Tips

- Start with a deterministic local fixture path before external integration.
- Add bounded concurrency controls early to prevent overload and flakiness.
- Separate collection from aggregation/output so each can be tested in isolation.
- Add backoff and retry policy tests for transient failures.
- Measure throughput and tail latency on representative input sizes.
