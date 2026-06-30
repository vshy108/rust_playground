# TODO: search_indexer (⭐ 7/10)

## Usage

```bash
cargo run --bin search_indexer
cargo test --bin search_indexer
```

## Milestones

- [ ] Crawl documents from a directory tree.
- [ ] Build an inverted index for terms to document IDs.
- [ ] Add simple query parsing with AND or OR behavior.
- [ ] Rank matches using a lightweight scoring rule.
- [ ] Add tests for tokenization, indexing, and querying.

## Extra

- [ ] Add incremental reindexing for changed files.

## Tips

- Keep tokenization deterministic so index tests stay stable.
- Build the index format with debugging visibility in mind.
