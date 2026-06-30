# TODO: wiki_engine (⭐ 7/10)

## Usage

```bash
cargo run --bin wiki_engine
cargo test --bin wiki_engine
```

## Milestones

- [ ] Implement page storage and revision history model.
- [ ] Add markdown or wiki-markup rendering pipeline.
- [ ] Implement internal link parsing, backlinks, and rename handling.
- [ ] Add search index for titles and page content.
- [ ] Implement edit conflict detection and revision diff support.
- [ ] Add tests for link graph updates, revision history, and render correctness.

## Extra

- [ ] Add attachment or image embedding support.

## Tips

- Title normalization and link resolution need deterministic rules.
- Revision history should be append-only and diffable.
- Backlinks are derived data; keep them rebuildable.
- Rendering and storage should remain separate concerns.
