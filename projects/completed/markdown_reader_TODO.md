# TODO: markdown_reader (⭐ 4/10)

## Usage

```bash
cargo run --bin markdown_reader
cargo test --bin markdown_reader
```

## Milestones

- [x] Load markdown files and render plain terminal output.
- [x] Implement heading, list, and code-block formatting rules.
- [x] Add simple navigation or section jump support.
- [x] Handle wide lines and wrapping predictably.
- [x] Add tests for rendering of representative markdown fixtures.

## Extra

- [x] Add table-of-contents generation.

## Status

Completed.

## Specification

- Goal: render a readable deterministic subset of Markdown for terminal use.
- Inputs: a Markdown file with optional width, section, or table-of-contents flags.
- Output: plain text headings, lists, indented code blocks, wrapped paragraphs,
  section slices, or a generated table of contents.
- Non-goals: full CommonMark compatibility, inline styling, images, and links.
- Acceptance: representative rendering, wrapping, section navigation, and TOC
  tests pass with strict Clippy enabled.

## Change record

- Implemented file loading, deterministic rendering rules, wrapping, section
  selection, TOC generation, and focused fixture-style tests.

## Tips

- Keep parsing and terminal rendering separate.
- Wrapping behavior needs deterministic tests.
- Code blocks and headings define most of the visible output quality.
