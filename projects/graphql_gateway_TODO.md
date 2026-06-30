# TODO: graphql_gateway (⭐ 8/10)

## Usage

```bash
cargo run --bin graphql_gateway
cargo test --bin graphql_gateway
```

## Milestones

- [ ] Model schema composition and resolver registry.
- [ ] Implement query parsing and execution planning.
- [ ] Add downstream data-source adapters and batching support.
- [ ] Implement auth context propagation and field-level checks.
- [ ] Add caching or persisted-query support.
- [ ] Add tests for schema validation, resolver errors, and batching behavior.

## Extra

- [ ] Add subscriptions over WebSocket.

## Tips

- Schema modeling and execution planning should be independent layers.
- N+1 behavior appears quickly; design batching hooks early.
- Field-level auth must not be hidden inside unrelated resolver code.
- Persisted-query support changes caching and security assumptions.
