# TODO: audit_trail_store (⭐ 6/10)

## Usage

```bash
cargo run --bin audit_trail_store
cargo test --bin audit_trail_store
```

## Milestones

- [ ] Define append-only audit event schema.
- [ ] Implement durable write path and sequence IDs.
- [ ] Add filtered query by actor, action, and time range.
- [ ] Support tamper-evidence via hash chaining basics.
- [ ] Add tests for ordering, filtering, and integrity checks.

## Extra

- [ ] Add export to signed archival bundles.

## Tips

- Append-only storage becomes simpler when updates are forbidden by design.
- Keep index structures optional so write path stays fast.
