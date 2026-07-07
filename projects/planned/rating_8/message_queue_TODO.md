# TODO: message_queue (⭐ 8/10)

## Usage

```bash
cargo run --bin message_queue
cargo test --bin message_queue
```

## Milestones

- [ ] Implement topic and partition abstractions.
- [ ] Add producer append path with ordering guarantees.
- [ ] Add consumer groups with offset tracking.
- [ ] Implement acknowledgment and redelivery semantics.
- [ ] Add retention policy and segment cleanup.
- [ ] Add tests for ordering, consumer failover, and replay.

## Extra

- [ ] Add dead-letter queue and delayed delivery support.

## Tips

- Model queue logs as append-only segments from the beginning.
- Keep consumer offsets in durable storage independent from message data.
- Define delivery semantics (at-most-once or at-least-once) explicitly.
- Stress test with concurrent producers to detect ordering bugs.
