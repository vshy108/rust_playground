# TODO: vector_search_engine (⭐ 9/10)

## Usage

```bash
cargo run --bin vector_search_engine
cargo test --bin vector_search_engine
```

## Milestones

- [ ] Define embedding storage format and collection metadata.
- [ ] Implement brute-force nearest-neighbor search first.
- [ ] Add approximate index structure such as HNSW or IVF.
- [ ] Implement filtering by metadata alongside vector similarity.
- [ ] Add persistence and index rebuild/recovery support.
- [ ] Add tests for distance correctness, recall, and filtered search behavior.

## Extra

- [ ] Add hybrid keyword + vector ranking.

## Tips

- Start from exact search to validate scoring before approximation.
- Distance metrics should be isolated and test-covered independently.
- Metadata filtering changes index strategy; model it intentionally.
- Persist enough metadata to rebuild approximate indexes safely.
