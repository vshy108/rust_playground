# TODO: markdown_parser (⭐ 5/10)

## Usage

```bash
cargo run --bin markdown_parser
cargo test --bin markdown_parser
```

## Milestones

- [ ] Parse paragraphs, emphasis, headings, and code spans.
- [ ] Add list and blockquote parsing with nesting rules.
- [ ] Build an AST that preserves block and inline structure.
- [ ] Render the AST to HTML.
- [ ] Add error-tolerant handling for malformed markdown.
- [ ] Add snapshot tests for representative CommonMark snippets.

## Extra

- [ ] Add fenced code blocks and link reference definitions.

## Tips

- Freeze a tiny grammar subset first and ship that end-to-end.
- Keep phases isolated: tokenize, parse, validate, execute/render.
- Add golden tests for AST/IR snapshots to catch accidental regressions.
- Prefer precise parse errors with position info before adding new syntax.
- Build tooling hooks early (debug dump, trace mode, disassemble output).
