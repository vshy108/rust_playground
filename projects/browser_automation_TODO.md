# TODO: browser_automation (⭐ 8/10)

## Usage

```bash
cargo run --bin browser_automation
cargo test --bin browser_automation
```

## Milestones

- [ ] Model browser session, page, and action abstractions.
- [ ] Implement navigation and wait conditions.
- [ ] Add DOM query and interaction primitives.
- [ ] Implement screenshot, HTML dump, or trace capture helpers.
- [ ] Add retry semantics for transient page timing issues.
- [ ] Add tests for action planning, selector failures, and wait-condition logic.

## Extra

- [ ] Add workflow scripting layer for reusable automation recipes.

## Tips

- Session orchestration and DOM targeting should be separate layers.
- Wait conditions are core product behavior, not just convenience helpers.
- Deterministic fixtures or mocked action graphs help before live browser coupling.
- Failure reporting should explain which action or selector broke.
