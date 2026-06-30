# TODO: mqtt_broker (⭐ 7/10)

## Usage

```bash
cargo run --bin mqtt_broker
cargo test --bin mqtt_broker
```

## Milestones

- [ ] Parse MQTT CONNECT, PUBLISH, SUBSCRIBE, and PING messages.
- [ ] Implement topic tree and wildcard matching (+ and #).
- [ ] Build client session lifecycle with keepalive timeouts.
- [ ] Add QoS0 and QoS1 delivery with packet identifiers.
- [ ] Add retained messages and last-will handling.
- [ ] Add tests for protocol decoding and topic routing behavior.

## Extra

- [ ] Add persistent session storage and replay after restart.

## Tips

- Start with protocol and contract tests first (request, response, error, timeout).
- Build a strict parser before adding convenience behavior; fail closed on malformed input.
- Add structured request logging early so debugging network paths is cheap.
- Keep connection lifecycle explicit: open, active, idle timeout, close.
- Add load and latency checks after correctness is stable.
