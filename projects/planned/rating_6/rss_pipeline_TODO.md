# TODO: rss_pipeline (⭐ 6/10)

## Usage

```bash
cargo run --bin rss_pipeline
cargo test --bin rss_pipeline
```

## Milestones

- [ ] Implement feed fetch and parse pipeline for RSS or Atom.
- [ ] Add pluggable transformation stages for filtering, enrichment, or rewriting.
- [ ] Implement deduplication of seen items across runs.
- [ ] Add output sinks such as files, webhooks, or another feed.
- [ ] Implement scheduling or backoff for polling remote feeds.
- [ ] Add tests for item deduplication, transform ordering, and malformed feed handling.

## Extra

- [ ] Add Markdown rendering or summarization stage.

## Tips

- Fetch, parse, transform, and sink steps should be independently testable.
- Deduplication keys need stable rules across feed edits.
- Backoff and polling cadence matter for real feed behavior.
- Preserve original item metadata even when transforms rewrite content.
