# TODO: mailing_list_manager (⭐ 8/10)

## Usage

```bash
cargo run --bin mailing_list_manager
cargo test --bin mailing_list_manager
```

## Milestones

- [ ] Model subscribers, lists, and moderation state.
- [ ] Implement subscribe/unsubscribe and confirmation flows.
- [ ] Add inbound post routing to list members.
- [ ] Implement moderation queue and approval actions.
- [ ] Add bounce handling or delivery suppression rules.
- [ ] Add tests for subscription lifecycle, moderation, and fanout behavior.

## Extra

- [ ] Add digest delivery mode or per-subscriber preferences.

## Tips

- Treat membership management and message delivery as separate subsystems.
- Confirmation and unsubscribe links are part of the core workflow.
- Delivery fanout is easier to reason about with deterministic fixtures.
- Moderation state transitions should be explicit and auditable.
