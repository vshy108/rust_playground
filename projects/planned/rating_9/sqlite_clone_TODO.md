# TODO: sqlite_clone (⭐ 9/10)

## Usage

```bash
cargo run --bin sqlite_clone
cargo test --bin sqlite_clone
```

## Milestones

- [ ] Build a REPL that accepts `.meta` commands and a tiny SQL subset.
- [ ] Parse `insert` and `select` into a small statement representation.
- [ ] Add row serialization and page-based persistence to disk.
- [ ] Implement a cursor abstraction over table pages.
- [ ] Add a B-tree leaf node format and binary search by primary key.
- [ ] Add tests for persistence, duplicate keys, and multi-page scans.

## Extra

- [ ] Add internal node splitting and secondary index experiments.

## Tips

- Lock down data invariants first and encode them in tests.
- Implement persistence boundaries early (snapshot, log, recovery) even if minimal.
- Separate correctness path from optimization path; optimize only after passing invariants.
- Add deterministic simulation tests for retries, crashes, and restart behavior.
- Track state transitions with trace logs to simplify post-failure analysis.
