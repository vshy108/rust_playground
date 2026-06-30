# TODO: http_load_tester (⭐ 5/10)

## Usage

```bash
cargo run --bin loadtest -- --url http://127.0.0.1:3000 --concurrency 32 --requests 2000
cargo test --bin loadtest
```

## Milestones

- [ ] Parse CLI options: URL, concurrency, total requests, timeout.
- [ ] Build async worker pool for request execution.
- [ ] Measure per-request latency and status code distribution.
- [ ] Print throughput, p50/p95/p99 latency, and error rate.
- [ ] Add connection reuse with shared client.
- [ ] Add deterministic tests around histogram math.

## Extra

- [ ] Add JSON report output.

## Tips

- Start with a deterministic local fixture path before external integration.
- Add bounded concurrency controls early to prevent overload and flakiness.
- Separate collection from aggregation/output so each can be tested in isolation.
- Add backoff and retry policy tests for transient failures.
- Measure throughput and tail latency on representative input sizes.
