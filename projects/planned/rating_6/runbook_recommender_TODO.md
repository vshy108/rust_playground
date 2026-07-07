# TODO: runbook_recommender (⭐ 6/10)

## Usage

```bash
cargo run --bin runbook_recommender
cargo test --bin runbook_recommender
```

## Milestones

- [ ] Index runbooks by tags, systems, and failure modes.
- [ ] Match incidents to candidate runbooks.
- [ ] Rank recommendations using historical outcomes.
- [ ] Capture operator feedback for ranking updates.
- [ ] Add tests for matching precision and ranking behavior.

## Extra

- [ ] Add contextual snippets for first-response guidance.

## Tips

- Keep retrieval and ranking stages independent.
- Normalize tags early to reduce sparse matching issues.
