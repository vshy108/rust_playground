# TODO: job_queue (⭐ 6/10)

## Usage

```bash
cargo run --bin job_queue
cargo test --bin job_queue
```

## Milestones

- [ ] Define `Job` model: id, payload, attempt, visibility deadline.
- [ ] Implement enqueue/dequeue/ack primitives.
- [ ] Implement worker loop with retry and backoff.
- [ ] Add dead-letter queue after max attempts.
- [ ] Add graceful shutdown and drain semantics.
- [ ] Add integration tests for retry + DLQ behavior.

## Extra

- [ ] Add durable WAL persistence.

## Tips

- Lock down data invariants first and encode them in tests.
- Implement persistence boundaries early (snapshot, log, recovery) even if minimal.
- Separate correctness path from optimization path; optimize only after passing invariants.
- Add deterministic simulation tests for retries, crashes, and restart behavior.
- Track state transitions with trace logs to simplify post-failure analysis.
