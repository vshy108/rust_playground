# TODO: notebook_sync_engine (⭐ 7/10)

## Usage

```bash
cargo run --bin notebook_sync_engine
cargo test --bin notebook_sync_engine
```

## Milestones

- [ ] Model notebook pages and local change sets.
- [ ] Detect divergent edits between two notebook states.
- [ ] Implement a simple merge or conflict marker strategy.
- [ ] Persist sync checkpoints and replay behavior.
- [ ] Add tests for conflict detection and merge outcomes.

## Extra

- [ ] Add file-watcher driven auto-sync.

## Tips

- Start with two-way sync for one notebook store before generalizing.
- A small explicit conflict model will prevent hidden merge bugs.
