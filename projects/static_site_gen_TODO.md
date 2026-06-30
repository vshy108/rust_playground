# TODO: static_site_gen (⭐ 8/10)

## Usage

```bash
cargo run --bin static_site_gen
cargo test --bin static_site_gen
```

## Milestones

- [ ] Parse markdown pages and front matter metadata.
- [ ] Build template rendering with layout inheritance.
- [ ] Generate route map and output directory structure.
- [ ] Add incremental rebuild based on file hash manifest.
- [ ] Add RSS feed and sitemap generation.
- [ ] Add tests for content parsing and deterministic output.

## Extra

- [ ] Add development server with live reload support.

## Tips

- Start with a deterministic local fixture path before external integration.
- Add bounded concurrency controls early to prevent overload and flakiness.
- Separate collection from aggregation/output so each can be tested in isolation.
- Add backoff and retry policy tests for transient failures.
- Measure throughput and tail latency on representative input sizes.
