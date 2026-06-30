# TODO: package_manager (⭐ 8/10)

## Usage

```bash
cargo run --bin package_manager
cargo test --bin package_manager
```

## Milestones

- [ ] Design package manifest format and dependency graph model.
- [ ] Implement a local package registry and metadata index.
- [ ] Add version resolution with semantic version constraints.
- [ ] Implement download/extract/install workflow with lockfile output.
- [ ] Add integrity verification with checksums for fetched artifacts.
- [ ] Add tests for resolver conflicts, lockfile stability, and reinstall behavior.

## Extra

- [ ] Add remote registry protocol and cache eviction policy.

## Tips

- Implement the dependency resolver independently from network code first.
- Lockfile generation should be deterministic to simplify review and caching.
- Cache package archives and unpacked artifacts separately.
- Validate checksum mismatches as explicit failure paths.
