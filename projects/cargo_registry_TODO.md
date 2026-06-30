# TODO: cargo_registry (⭐ 8/10)

## Usage

```bash
cargo run --bin cargo_registry
cargo test --bin cargo_registry
```

## Milestones

- [ ] Design crate index format and package metadata storage.
- [ ] Implement publish flow with version conflict handling.
- [ ] Add artifact storage for crate tarballs and checksums.
- [ ] Implement crate fetch/index query endpoints.
- [ ] Add auth, namespace, or access policy hooks.
- [ ] Add tests for publish idempotency, corrupted uploads, and index consistency.

## Extra

- [ ] Add upstream proxying and cache mode for crates.io mirroring.

## Tips

- Index correctness matters more than UI or admin features at first.
- Publishing should be atomic: index and tarball updates must agree.
- Treat checksum verification as mandatory, not optional hardening.
- Small fixture crates make resolver and fetch tests much easier.
