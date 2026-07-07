# TODO: lsm_kv (⭐ 8/10)

## Usage

```bash
cargo run --bin lsm_kv
cargo test --bin lsm_kv
```

## Milestones

- [ ] Implement memtable put/get/delete.
- [ ] Implement write-ahead log for crash recovery.
- [ ] Flush immutable memtable to SSTable files.
- [ ] Implement read path across memtable + SSTables.
- [ ] Implement compaction strategy and tombstone handling.
- [ ] Add recovery tests across restart.

## Extra

- [ ] Add bloom filters per SSTable.

## Tips

- Lock down data invariants first and encode them in tests.
- Implement persistence boundaries early (snapshot, log, recovery) even if minimal.
- Separate correctness path from optimization path; optimize only after passing invariants.
- Add deterministic simulation tests for retries, crashes, and restart behavior.
- Track state transitions with trace logs to simplify post-failure analysis.
