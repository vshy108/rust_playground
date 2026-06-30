# TODO: dns_toolkit (⭐ 5/10)

## Usage

```bash
cargo run --bin dns_toolkit
cargo test --bin dns_toolkit
```

## Milestones

- [ ] Parse and encode DNS header + question sections (byte-level codec).
- [ ] Build a UDP resolver command: `resolve <name> <type>`.
- [ ] Add in-memory cache with TTL-aware expiration.
- [ ] Add tiny authoritative mode for a local zone file.
- [ ] Add retries + timeout backoff for upstream queries.
- [ ] Add tests for parsing, encoding, and cache expiry.

## Extra

- [ ] DNS over HTTPS client path (optional flag).

## Tips

- Start with protocol and contract tests first (request, response, error, timeout).
- Build a strict parser before adding convenience behavior; fail closed on malformed input.
- Add structured request logging early so debugging network paths is cheap.
- Keep connection lifecycle explicit: open, active, idle timeout, close.
- Add load and latency checks after correctness is stable.
