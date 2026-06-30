# TODO: socks5_proxy (⭐ 7/10)

## Usage

```bash
cargo run --bin socks5_proxy -- --listen 127.0.0.1:1080
cargo test --bin socks5_proxy
```

## Milestones

- [ ] Implement SOCKS5 greeting and method selection.
- [ ] Implement CONNECT request parsing.
- [ ] Dial upstream and start bidirectional copy.
- [ ] Handle domain, IPv4, IPv6 target addressing.
- [ ] Add timeouts and connection metrics.
- [ ] Add protocol conformance tests.

## Extra

- [ ] Add username/password auth mode.

## Tips

- Start with protocol and contract tests first (request, response, error, timeout).
- Build a strict parser before adding convenience behavior; fail closed on malformed input.
- Add structured request logging early so debugging network paths is cheap.
- Keep connection lifecycle explicit: open, active, idle timeout, close.
- Add load and latency checks after correctness is stable.
