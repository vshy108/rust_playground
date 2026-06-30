# TODO: container_registry (⭐ 8/10)

## Usage

```bash
cargo run --bin container_registry
cargo test --bin container_registry
```

## Milestones

- [ ] Implement OCI manifest and blob metadata models.
- [ ] Add blob upload/download endpoints with digest validation.
- [ ] Implement repository and tag index semantics.
- [ ] Add auth hooks and namespace access checks.
- [ ] Implement garbage collection for unreferenced blobs.
- [ ] Add tests for digest mismatches, manifest/tag consistency, and concurrent upload flows.

## Extra

- [ ] Add registry proxy cache mode for upstream images.

## Tips

- Treat digest verification as part of the normal write path.
- Blob storage and tag bookkeeping should be decoupled.
- Partial uploads need explicit cleanup rules.
- Manifest/tag races are common; model them directly in tests.
