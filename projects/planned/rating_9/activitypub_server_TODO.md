# TODO: activitypub_server (⭐ 9/10)

## Usage

```bash
cargo run --bin activitypub_server
cargo test --bin activitypub_server
```

## Milestones

- [ ] Model actors, inboxes, outboxes, and object identifiers.
- [ ] Implement ActivityPub JSON serialization and signature verification flow.
- [ ] Add delivery queue for outbound federation.
- [ ] Implement inbox processing for follow, accept, create, and delete activities.
- [ ] Add object persistence and remote actor caching.
- [ ] Add tests for signature checks, idempotent delivery, and federated state transitions.

## Extra

- [ ] Add webfinger discovery and remote instance health checks.

## Tips

- Stable identifiers and inbox semantics define the protocol surface.
- Federation needs retries and idempotency from the start.
- Keep local actor state separate from remote cache state.
- Protocol fixtures from real ActivityPub examples will help avoid wire-format drift.
