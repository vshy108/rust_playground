# TODO: regex_engine (⭐ 9/10)

## Usage

```bash
cargo run --bin regex_engine -- "a+b" "aaab"
cargo test --bin regex_engine
```

## Milestones

- [ ] Parse a regex grammar into AST.
- [ ] Compile AST into NFA.
- [ ] Implement NFA simulation matcher.
- [ ] Add character classes and escaped tokens.
- [ ] Add anchors and quantifiers.
- [ ] Add conformance tests against known examples.

## Extra

- [ ] Add NFA -> DFA subset construction path.

## Tips

- Freeze a tiny grammar subset first and ship that end-to-end.
- Keep phases isolated: tokenize, parse, validate, execute/render.
- Add golden tests for AST/IR snapshots to catch accidental regressions.
- Prefer precise parse errors with position info before adding new syntax.
- Build tooling hooks early (debug dump, trace mode, disassemble output).
