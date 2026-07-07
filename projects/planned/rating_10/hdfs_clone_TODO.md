# TODO: hdfs_clone (⭐ 10/10)

## Usage

```bash
cargo run --bin hdfs_clone
cargo test --bin hdfs_clone
```

## Milestones

- [ ] Model namenode metadata and datanode block ownership.
- [ ] Implement file create/read pipeline with block splitting.
- [ ] Add replication placement and recovery rules.
- [ ] Implement heartbeat and block report flows from datanodes.
- [ ] Add rebalancing or under-replicated block repair behavior.
- [ ] Add tests for metadata consistency, node failure recovery, and read-path correctness.

## Extra

- [ ] Add rack-aware placement and balancer heuristics.

## Tips

- Metadata correctness is the primary system invariant; isolate it from block I/O.
- Failure scenarios define distributed storage systems more than happy-path writes.
- Replication repair needs explicit, deterministic tests.
- Start with a single process simulation before real networking.
