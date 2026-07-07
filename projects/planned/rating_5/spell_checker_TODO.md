# TODO: spell_checker (⭐ 5/10)

## Usage

```bash
cargo run --bin spell_checker
cargo test --bin spell_checker
```

## Milestones

- [ ] Load a dictionary and normalize words for lookup.
- [ ] Implement exact-word membership checks.
- [ ] Add edit-distance or candidate-generation suggestions.
- [ ] Rank suggestions by distance and frequency.
- [ ] Add a CLI for checking text input or files.
- [ ] Add tests for normalization, candidate generation, and ranking.

## Extra

- [ ] Add trie-based prefix suggestions and autocomplete mode.

## Tips

- Freeze a tiny grammar subset first and ship that end-to-end.
- Keep phases isolated: tokenize, parse, validate, execute/render.
- Add golden tests for AST/IR snapshots to catch accidental regressions.
- Prefer precise parse errors with position info before adding new syntax.
- Build tooling hooks early (debug dump, trace mode, disassemble output).
