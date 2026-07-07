# TODO: template_engine (⭐ 8/10)

## Usage

```bash
cargo run --bin template_engine
cargo test --bin template_engine
```

## Milestones

- [ ] Define token types for text/variable/control blocks.
- [ ] Build lexer for `{{ }}` and `{% %}` syntax.
- [ ] Build parser into a small AST.
- [ ] Implement renderer with context map lookup.
- [ ] Add loops and conditionals.
- [ ] Add snapshot tests for rendered output.

## Extra

- [ ] Add template include support with cycle detection.

## Tips

- Freeze a tiny grammar subset first and ship that end-to-end.
- Keep phases isolated: tokenize, parse, validate, execute/render.
- Add golden tests for AST/IR snapshots to catch accidental regressions.
- Prefer precise parse errors with position info before adding new syntax.
- Build tooling hooks early (debug dump, trace mode, disassemble output).
