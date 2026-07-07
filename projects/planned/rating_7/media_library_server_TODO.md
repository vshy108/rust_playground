# TODO: media_library_server (⭐ 7/10)

## Usage

```bash
cargo run --bin media_library_server
cargo test --bin media_library_server
```

## Milestones

- [ ] Scan a media directory and extract basic metadata.
- [ ] Store albums, artists, and tracks in an internal catalog.
- [ ] Serve a small browse API or HTML interface.
- [ ] Add search and sorting for the catalog.
- [ ] Add tests for scan results and metadata normalization.

## Extra

- [ ] Add streaming or cover-art endpoints.

## Tips

- Separate metadata extraction from serving so the index can evolve independently.
- Normalize names early to avoid duplicate artist or album records.
