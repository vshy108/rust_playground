# TODO: spell_checker (⭐ 5/10)

## Usage

```bash
cargo run --bin spell_checker
cargo test --bin spell_checker
```

## Milestones

- [x] Load a dictionary and normalize words for lookup.
- [x] Implement exact-word membership checks.
- [x] Add edit-distance or candidate-generation suggestions.
- [x] Rank suggestions by distance and frequency.
- [x] Add a CLI for checking text input or files.
- [x] Add tests for normalization, candidate generation, and ranking.

## Extra

- [x] Add trie-based prefix suggestions and autocomplete mode.

## Status

Completed.

## Specification

- Goal: check text against a frequency-aware normalized dictionary.
- Inputs: dictionary entries, text files, and optional autocomplete prefixes.
- Output: unknown words with ranked suggestions or prefix completions.
- Errors: report missing files, malformed frequencies, and invalid CLI shapes.
- Non-goals: language-specific morphology, phonetic matching, and a persistent
  trie index.
- Acceptance: normalization, distance, ranking, autocomplete, focused tests,
  and strict Clippy checks pass.

## Change record

- Implemented dictionary loading, token normalization, Levenshtein distance,
  frequency-aware suggestions, file checking, and autocomplete mode.

## Tips

- Freeze a tiny grammar subset first and ship that end-to-end.
- Keep phases isolated: tokenize, parse, validate, execute/render.
- Add golden tests for AST/IR snapshots to catch accidental regressions.
- Prefer precise parse errors with position info before adding new syntax.
- Build tooling hooks early (debug dump, trace mode, disassemble output).
