# TODO: package_registry (⭐ 8/10)

## Usage

```bash
cargo run --bin package_registry
cargo test --bin package_registry
```

## Milestones

- [ ] Model packages, versions, and immutable artifacts.
- [ ] Add upload and download endpoints with metadata checks.
- [ ] Serve an index or manifest format for clients.
- [ ] Add authentication and namespace ownership basics.
- [ ] Add tests for publish, fetch, and conflict behavior.

## Extra

- [ ] Add upstream proxying and local caching.

## Tips

- Keep artifact storage and index mutation clearly separated.
- Immutable version rules should be enforced centrally, not per handler.
