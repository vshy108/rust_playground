# TODO: notification_hub (⭐ 5/10)

## Usage

```bash
cargo run --bin notification_hub
cargo test --bin notification_hub
```

## Milestones

- [ ] Accept notifications from multiple input sources.
- [ ] Route messages by channel, topic, or severity.
- [ ] Add sink adapters like stdout, file, or webhook.
- [ ] Support retry or dead-letter basics.
- [ ] Add tests for routing and delivery behavior.

## Extra

- [ ] Add quiet hours or deduplication rules.

## Tips

- Model a notification envelope first so routes and sinks stay generic.
- Delivery policies should be separate from channel matching.
