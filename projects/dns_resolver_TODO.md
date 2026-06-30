# TODO: dns_resolver (⭐ 7/10)

## Usage

```bash
cargo run --bin dns_resolver
cargo test --bin dns_resolver
```

## Milestones

- [ ] Implement DNS packet parsing and serialization.
- [ ] Add UDP query handling with recursive resolution flow.
- [ ] Implement local cache with TTL expiry.
- [ ] Add support for common record types and NXDOMAIN handling.
- [ ] Implement timeout, retry, and upstream fallback behavior.
- [ ] Add tests for packet correctness, caching, and recursive lookup behavior.

## Extra

- [ ] Add DNS-over-TLS or DNS-over-HTTPS support.

## Tips

- Packet correctness matters more than optimization in the first pass.
- Isolate parser/serializer logic so it can be fuzzed or fixture-tested.
- Cache entries need expiry semantics independent from transport retries.
- Build fixtures for known answers and malformed packets.
