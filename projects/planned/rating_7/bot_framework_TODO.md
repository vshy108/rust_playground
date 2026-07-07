# TODO: bot_framework (⭐ 7/10)

## Usage

```bash
cargo run --bin bot_framework
cargo test --bin bot_framework
```

## Milestones

- [ ] Model bot runtime, event handlers, and command dispatch.
- [ ] Add adapter abstraction for chat or webhook providers.
- [ ] Implement middleware chain for auth, logging, and rate limiting.
- [ ] Add state store for conversations or bot workflows.
- [ ] Implement retry and dead-letter handling for failed actions.
- [ ] Add tests for event routing, middleware ordering, and adapter isolation.

## Extra

- [ ] Add a small scripting layer for declarative bot workflows.

## Tips

- Provider adapters should not leak transport details into bot logic.
- Event routing and action execution need clear isolation for tests.
- Middleware order affects correctness, not just style.
- Conversation state should survive handler refactors.
