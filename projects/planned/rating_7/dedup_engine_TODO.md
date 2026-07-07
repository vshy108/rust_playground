# TODO: dedup_engine (⭐ 7/10)

## Usage

```bash
cargo run --bin dedup_engine
cargo test --bin dedup_engine
```

## Milestones

- [ ] Implement file discovery and content chunking strategy.
- [ ] Add strong hash indexing for full-file and chunk-level deduplication.
- [ ] Implement duplicate grouping and reclaimable-space reporting.
- [ ] Add safe linking or copy-on-write replacement plan generation.
- [ ] Implement exclusion rules and dry-run mode.
- [ ] Add tests for duplicate detection, collision handling, and path safety rules.

## Extra

- [ ] Add rolling-hash chunking for content-defined deduplication.

## Tips

- Discovery, hashing, and replacement planning should be separate stages.
- Safety checks matter more than aggressive dedup wins.
- Dry-run output should explain exactly what would be replaced.
- Chunk-level dedup is a later layer; start with whole-file correctness first.
