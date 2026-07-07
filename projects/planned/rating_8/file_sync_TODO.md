# TODO: file_sync (⭐ 8/10)

## Usage

```bash
cargo run --bin file_sync -- --src ./fixtures --dst /tmp/sync_target
cargo test --bin file_sync
```

## Milestones

- [ ] Build directory scanner and manifest model.
- [ ] Compute content hashes and compare manifests.
- [ ] Copy new/changed files and prune deleted files.
- [ ] Add resume-safe temp-file writes + atomic rename.
- [ ] Add parallel worker pool with bounded concurrency.
- [ ] Add tests for rename/update/delete scenarios.

## Extra

- [ ] Add rsync-style rolling checksum delta mode.

## Tips

- Start with a deterministic local fixture path before external integration.
- Add bounded concurrency controls early to prevent overload and flakiness.
- Separate collection from aggregation/output so each can be tested in isolation.
- Add backoff and retry policy tests for transient failures.
- Measure throughput and tail latency on representative input sizes.
