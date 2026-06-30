# TODO: env_linter (⭐ 3/10)

## Usage

```bash
cargo run --bin env_linter
cargo test --bin env_linter
```

## Milestones

- [ ] Parse `.env` style key-value files.
- [ ] Detect common issues such as duplicate keys or invalid names.
- [ ] Add missing-value or whitespace warning rules.
- [ ] Implement human-readable and machine-readable output.
- [ ] Add tests for rule triggering and parser edge cases.

## Extra

- [ ] Add autofix suggestions for simple issues.

## Tips

- Keep parsing separate from lint rule evaluation.
- Duplicate-key handling should preserve first-seen location info.
