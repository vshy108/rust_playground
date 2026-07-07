# TODO: markdown_reader (⭐ 4/10)

## Usage

```bash
cargo run --bin markdown_reader
cargo test --bin markdown_reader
```

## Milestones

- [ ] Load markdown files and render plain terminal output.
- [ ] Implement heading, list, and code-block formatting rules.
- [ ] Add simple navigation or section jump support.
- [ ] Handle wide lines and wrapping predictably.
- [ ] Add tests for rendering of representative markdown fixtures.

## Extra

- [ ] Add table-of-contents generation.

## Tips

- Keep parsing and terminal rendering separate.
- Wrapping behavior needs deterministic tests.
- Code blocks and headings define most of the visible output quality.
