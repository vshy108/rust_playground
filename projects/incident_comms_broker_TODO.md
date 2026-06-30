# TODO: incident_comms_broker (⭐ 6/10)

## Usage

```bash
cargo run --bin incident_comms_broker
cargo test --bin incident_comms_broker
```

## Milestones

- [ ] Model incident channels and subscriber policies.
- [ ] Route updates across chat, email, and webhook sinks.
- [ ] Enforce message deduplication and ordering.
- [ ] Support escalation templates by incident severity.
- [ ] Add tests for fan-out and delivery retries.

## Extra

- [ ] Add stakeholder-specific update digests.

## Tips

- Use idempotency keys to prevent duplicate broadcasts.
- Keep transport adapters isolated from routing logic.
