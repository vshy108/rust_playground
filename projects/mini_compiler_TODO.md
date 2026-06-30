# TODO: mini_compiler (⭐ 8/10)

## Usage

```bash
cargo run --bin mini_compiler
cargo test --bin mini_compiler
```

## Milestones

- [ ] Define a small expression language grammar.
- [ ] Implement lexer and recursive descent parser.
- [ ] Build typed AST and semantic validation pass.
- [ ] Implement bytecode or IR code generation.
- [ ] Execute generated code in a small runtime.
- [ ] Add tests for parsing, type errors, and execution correctness.

## Extra

- [ ] Add optimization passes (constant folding + dead code removal).

## Tips

- Freeze a tiny grammar subset first and ship that end-to-end.
- Keep phases isolated: tokenize, parse, validate, execute/render.
- Add golden tests for AST/IR snapshots to catch accidental regressions.
- Prefer precise parse errors with position info before adding new syntax.
- Build tooling hooks early (debug dump, trace mode, disassemble output).
