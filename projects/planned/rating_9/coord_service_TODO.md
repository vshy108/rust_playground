# TODO: coord_service (⭐ 9/10)

## Usage

```bash
cargo run --bin coord_service
cargo test --bin coord_service
```

## Milestones

- [ ] Implement hierarchical keyspace or lease-backed key model.
- [ ] Add ephemeral nodes or lease expiration semantics.
- [ ] Implement watch subscriptions for key changes.
- [ ] Add distributed lock and leader-election primitives.
- [ ] Add snapshot/recovery or replicated log persistence.
- [ ] Add tests for lease expiry, lock contention, and watcher delivery order.

## Extra

- [ ] Add Raft-backed replication and quorum reads.

## Tips

- Define consistency rules up front: linearizable, sequential, or eventual.
- Model leases and sessions independently from stored values.
- Watch delivery ordering matters more than raw throughput at first.
- Start with single-node coordination semantics before replication.
