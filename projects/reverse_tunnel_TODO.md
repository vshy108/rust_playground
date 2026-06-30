# TODO: reverse_tunnel (⭐ 8/10)

## Usage

```bash
cargo run --bin reverse_tunnel
cargo test --bin reverse_tunnel
```

## Milestones

- [ ] Implement client and server session handshake.
- [ ] Add remote port registration and access policy checks.
- [ ] Implement multiplexed stream forwarding over a persistent control channel.
- [ ] Add reconnect and session resume behavior.
- [ ] Implement metrics or health reporting for forwarded sessions.
- [ ] Add tests for connection drops, port collisions, and stream routing correctness.

## Extra

- [ ] Add TLS or Noise-based mutual authentication.

## Tips

- Separate control-plane state from data-plane forwarding.
- Tunnel lifecycle edge cases matter more than happy-path byte shuffling.
- Stream multiplexing must be deterministic under reconnect pressure.
- Port reservation rules should be explicit and test-covered.
