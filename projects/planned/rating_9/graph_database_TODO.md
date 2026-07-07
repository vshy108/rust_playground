# TODO: graph_database (⭐ 9/10)

## Usage

```bash
cargo run --bin graph_database
cargo test --bin graph_database
```

## Milestones

- [ ] Implement node/edge storage model with typed properties.
- [ ] Add indexing for labels and frequently queried properties.
- [ ] Design a small graph query language for pattern matching.
- [ ] Implement traversal engine with filtering and projection.
- [ ] Add persistence with snapshot and recovery support.
- [ ] Add tests for traversals, index lookups, and query correctness.

## Extra

- [ ] Add shortest-path and centrality built-ins.

## Tips

- Keep storage IDs stable even when compaction runs.
- Query planner and execution should be decoupled for debugging.
- Start with single-threaded traversal semantics before parallel scans.
- Build fixtures with small but expressive graph shapes.
