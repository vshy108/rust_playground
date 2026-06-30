# TODO: kafka_consumer (⭐ 7/10)

## Usage

```bash
cargo run --bin kafka_consumer
cargo test --bin kafka_consumer
```

## 1. Message model

- [ ] Define a `Message` struct: `topic: String`, `key: String`, `payload: String`,
  `retry_count: u32`.
- [ ] Define a `ProcessResult` enum: `Ok`, `Retry(reason)`, `Poison(reason)`.

Acceptance check: structs compile; `ProcessResult` can be matched.

## 2. Simulated broker

- [ ] Use an in-memory `tokio::sync::mpsc` channel as a fake Kafka topic.
- [ ] Write a producer helper that sends N messages into the channel.

Acceptance check: producer sends 10 messages; consumer receives all 10.

## 3. Worker pool

- [ ] Spawn W worker tasks; each receives from a shared `Arc<Mutex<Receiver>>` or a
  broadcast/work-stealing channel.
- [ ] Each worker calls a `process(msg) -> ProcessResult` closure.

Acceptance check: 4 workers processing 100 messages finish faster than 1 worker.

## 4. Retry logic

- [ ] On `Retry`, re-enqueue the message with `retry_count += 1`.
- [ ] After max retries (e.g. 3), route to the dead-letter queue (DLQ).

Acceptance check: a message that always returns `Retry` ends up in the DLQ after 3 attempts.

## 5. DLQ

- [ ] Collect poisoned messages in a `Vec<Message>` (or a second channel).
- [ ] Print a DLQ summary at shutdown.

Acceptance check: DLQ contains the expected messages after processing.

## 6. Tests

- [ ] Successful message is processed exactly once.
- [ ] Retry exhaustion moves message to DLQ.
- [ ] Worker pool drains all messages before shutdown.

## Extra: tracing

- [ ] Add `tracing` crate; emit a `info_span!` per message; log retry attempts and DLQ moves.

## Tips

- Start with a deterministic local fixture path before external integration.
- Add bounded concurrency controls early to prevent overload and flakiness.
- Separate collection from aggregation/output so each can be tested in isolation.
- Add backoff and retry policy tests for transient failures.
- Measure throughput and tail latency on representative input sizes.
