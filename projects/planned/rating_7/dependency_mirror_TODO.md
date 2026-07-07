# TODO: dependency_mirror (⭐ 7/10)

## Usage

```bash
cargo run --bin dependency_mirror
cargo test --bin dependency_mirror
```

## Milestones

- [ ] Mirror package metadata and blobs from an upstream source.
- [ ] Implement cache indexing and freshness policies.
- [ ] Add integrity checks (digest verification).
- [ ] Support fallback to upstream on cache miss.
- [ ] Add tests for mirror consistency and failover behavior.

## Extra

- [ ] Add offline mode with periodic sync jobs.

## Tips

- Keep metadata and artifact stores decoupled to simplify consistency checks.
- Validate digests before serving cached artifacts.
