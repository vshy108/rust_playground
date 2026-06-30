# TODO: browser_engine (⭐ 8/10)

## Usage

```bash
cargo run --bin browser_engine
cargo test --bin browser_engine
```

## Milestones

- [ ] Build tokenizer and parser for a small HTML subset.
- [ ] Build CSS parser for selectors and declaration blocks.
- [ ] Construct DOM and style trees, then a layout tree.
- [ ] Implement block/inline layout with box model sizing.
- [ ] Add text rendering and simple paint command generation.
- [ ] Add tests for parsing, selector matching, and layout outputs.

## Extra

- [ ] Add image loading and incremental reflow on style changes.

## Tips

- Freeze a tiny grammar subset first and ship that end-to-end.
- Keep phases isolated: tokenize, parse, validate, execute/render.
- Add golden tests for AST/IR snapshots to catch accidental regressions.
- Prefer precise parse errors with position info before adding new syntax.
- Build tooling hooks early (debug dump, trace mode, disassemble output).
