# TODO: policy_engine (⭐ 7/10)

## Usage

```bash
cargo run --bin policy_engine
cargo test --bin policy_engine
```

## Milestones

- [ ] Design a small rule language or JSON-based policy format.
- [ ] Implement parser and AST for policy definitions.
- [ ] Add evaluator for subject, action, resource, and context checks.
- [ ] Implement effect resolution with allow/deny precedence.
- [ ] Add trace output to explain policy decisions.
- [ ] Add tests for precedence, missing context, and decision reproducibility.

## Extra

- [ ] Add partial evaluation or policy compilation for hot paths.

## Tips

- Favor explicit semantics over clever syntax in the first pass.
- Explanations are part of the product; add them early.
- Keep data model and evaluator isolated so policies can be fuzzed.
- Deterministic output matters if policies are audited or cached.
