# TODO: backup_tool (⭐ 6/10)

## Usage

```bash
cargo run --bin backup_tool
cargo test --bin backup_tool
```

## Milestones

- [ ] Walk source directories and collect files to archive.
- [ ] Copy or pack files into timestamped snapshots.
- [ ] Skip unchanged files using metadata or checksums.
- [ ] Add restore listing or restore-one-file behavior.
- [ ] Add tests for snapshot layout and exclusion rules.

## Extra

- [ ] Add compression and retention policies.

## Tips

- Decide whether this is snapshot-copy based or archive based before coding.
- Restore behavior is easier if snapshot manifests are explicit.
