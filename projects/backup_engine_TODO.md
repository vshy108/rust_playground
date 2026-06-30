# TODO: backup_engine (⭐ 8/10)

## Usage

```bash
cargo run --bin backup_engine
cargo test --bin backup_engine
```

## Milestones

- [ ] Implement filesystem walking and snapshot manifest generation.
- [ ] Add chunking and deduplication for file content.
- [ ] Implement encrypted or checksum-verified pack storage.
- [ ] Add restore flow for files and directory trees.
- [ ] Add retention policy or snapshot pruning.
- [ ] Add tests for dedup hits, restore correctness, and interrupted backups.

## Extra

- [ ] Add remote backend support and resumable uploads.

## Tips

- Separate snapshot metadata from chunk storage from the beginning.
- Restore correctness is the real acceptance bar; test it early.
- Keep path normalization deterministic across platforms.
- Make interrupted-run cleanup auditable, not implicit.
