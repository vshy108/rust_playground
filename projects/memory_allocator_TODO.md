# TODO: memory_allocator (⭐ 8/10)

## Usage

```bash
cargo run --bin memory_allocator
cargo test --bin memory_allocator
```

## Milestones

- [ ] Implement a bump allocator over a fixed memory region.
- [ ] Add free-list or linked-list allocation for reuse of freed blocks.
- [ ] Add alignment handling and split/coalesce behavior.
- [ ] Add allocation statistics and fragmentation metrics.
- [ ] Integrate with Rust allocator traits or a test harness abstraction.
- [ ] Add tests for alignment, fragmentation, and invalid free protection.

## Extra

- [ ] Compare bump, slab, and segregated-fit strategies.

## Tips

- Lock down data invariants first and encode them in tests.
- Implement persistence boundaries early (snapshot, log, recovery) even if minimal.
- Separate correctness path from optimization path; optimize only after passing invariants.
- Add deterministic simulation tests for retries, crashes, and restart behavior.
- Track state transitions with trace logs to simplify post-failure analysis.
