# TODO: websocket_broker (⭐ 7/10)

## Usage

```bash
cargo run --bin ws_broker
cargo test --bin ws_broker
```

## Milestones

- [ ] Implement WebSocket handshake endpoint.
- [ ] Add topic-based subscribe/unsubscribe protocol.
- [ ] Add publish flow and fan-out to subscribers.
- [ ] Handle slow consumers with bounded buffers.
- [ ] Add heartbeat/ping timeout and disconnect cleanup.
- [ ] Add integration tests with multiple clients.

## Extra

- [ ] Add retained messages per topic.

## Tips

- Start with protocol and contract tests first (request, response, error, timeout).
- Build a strict parser before adding convenience behavior; fail closed on malformed input.
- Add structured request logging early so debugging network paths is cheap.
- Keep connection lifecycle explicit: open, active, idle timeout, close.
- Add load and latency checks after correctness is stable.
