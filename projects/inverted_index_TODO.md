# TODO: inverted_index (⭐ 6/10)

## Usage

```bash
cargo run --bin inv_index -- fixtures/sample.txt
cargo test --bin inv_index
```

## Milestones

- [ ] Build tokenizer and normalization pipeline.
- [ ] Build inverted index map: term -> postings.
- [ ] Add document frequency and term frequency stats.
- [ ] Implement query parser for AND / OR terms.
- [ ] Add BM25-lite scoring and ranked output.
- [ ] Add tests for tokenizer, postings, and ranking.

## Extra

- [ ] Persist index to disk and reload quickly on startup.

## Tips

- Implement one milestone at a time and keep each slice testable.
- Add a failing test first, then implement the smallest behavior to pass.
- Keep CLI/API surface stable while iterating internals.
- Validate both happy path and error path for every milestone.
- Run focused tests before broad checks.
