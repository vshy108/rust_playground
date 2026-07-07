# TODO: object_store (⭐ 9/10)

## Usage

```bash
cargo run --bin object_store
cargo test --bin object_store
```

## Milestones

- [ ] Define bucket/object metadata model and content-addressed layout.
- [ ] Implement PUT/GET/DELETE flows with streaming file I/O.
- [ ] Add multipart upload and large-object chunk management.
- [ ] Add integrity verification with checksums and metadata validation.
- [ ] Implement compaction or garbage collection for unreferenced blobs.
- [ ] Add tests for overwrite semantics, partial failures, and restore paths.

## Extra

- [ ] Add S3-compatible API surface and signed request validation.

## Tips

- Separate metadata index from blob storage early so recovery stays tractable.
- Stream payloads instead of buffering entire objects in memory.
- Make checksum mismatches explicit first-class failures.
- Test crash-safe cleanup around multipart uploads.
