# TODO: vpn_tunnel (⭐ 9/10)

## Usage

```bash
cargo run --bin vpn_tunnel
cargo test --bin vpn_tunnel
```

## Milestones

- [ ] Model tunnel configuration, peers, and key material.
- [ ] Implement packet encapsulation and decapsulation flow.
- [ ] Add peer handshake and session establishment state.
- [ ] Add routing decisions for virtual interfaces and subnets.
- [ ] Add replay protection, keepalives, and rekey behavior.
- [ ] Add tests for packet parsing, key rotation, and peer state transitions.

## Extra

- [ ] Add a userspace TUN adapter for end-to-end local testing.

## Tips

- Start with protocol and contract tests first (request, response, error, timeout).
- Build a strict parser before adding convenience behavior; fail closed on malformed input.
- Add structured request logging early so debugging network paths is cheap.
- Keep connection lifecycle explicit: open, active, idle timeout, close.
- Add load and latency checks after correctness is stable.
