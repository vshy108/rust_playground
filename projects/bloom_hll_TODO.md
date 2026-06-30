# TODO: bloom_hll (⭐ 8/10)

## Usage

```bash
cargo run --bin bloom_hll
cargo test --bin bloom_hll
```

## Milestones

- [ ] Implement Bloom filter with configurable false-positive rate.
- [ ] Implement HyperLogLog cardinality estimator.
- [ ] Add CLI demo for insert/query/count.
- [ ] Add serialization format for both structures.
- [ ] Add error analysis against exact sets.
- [ ] Add tests for merge semantics.

## Extra

- [ ] Add Count-Min Sketch for frequency estimation.

## Tips

- Lock down data invariants first and encode them in tests.
- Implement persistence boundaries early (snapshot, log, recovery) even if minimal.
- Separate correctness path from optimization path; optimize only after passing invariants.
- Add deterministic simulation tests for retries, crashes, and restart behavior.
- Track state transitions with trace logs to simplify post-failure analysis.
