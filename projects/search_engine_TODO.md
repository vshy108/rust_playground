# TODO: search_engine (⭐ 8/10)

## Usage

```bash
cargo run --bin search_engine
cargo test --bin search_engine
```

## Milestones

- [ ] Define document schema and a tokenizer pipeline.
- [ ] Build an inverted index with posting lists and term statistics.
- [ ] Implement ranking with BM25-style scoring.
- [ ] Add query parser support for phrase and boolean terms.
- [ ] Add incremental indexing and segment merge workflow.
- [ ] Add tests for tokenization, ranking, and query behavior.

## Extra

- [ ] Add typo tolerance and snippet generation.

## Tips

- Keep tokenizer, indexer, and ranker as separate modules for easier testing.
- Start with in-memory segments before persisting index files.
- Add deterministic ranking fixtures to prevent accidental score regressions.
- Track index size and query latency as part of acceptance checks.
