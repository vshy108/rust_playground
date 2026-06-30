# TODO: dns_authority (⭐ 8/10)

## Usage

```bash
cargo run --bin dns_authority
cargo test --bin dns_authority
```

## Milestones

- [ ] Implement zone file or record storage model.
- [ ] Add authoritative lookup path for common record types.
- [ ] Implement SOA, NS, and negative response behavior.
- [ ] Add UDP request handling with truncated-response logic.
- [ ] Implement zone reload or dynamic update basics.
- [ ] Add tests for delegation boundaries, NXDOMAIN rules, and response encoding.

## Extra

- [ ] Add AXFR or IXFR-style zone transfer support.

## Tips

- Authoritative semantics differ from recursive resolution; keep them separate.
- Zone ownership and delegation boundaries define correctness.
- Response flags and authority sections deserve fixture-driven tests.
- Start with one zone and static records before dynamic update complexity.
