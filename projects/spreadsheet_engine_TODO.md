# TODO: spreadsheet_engine (⭐ 7/10)

## Usage

```bash
cargo run --bin spreadsheet_engine
cargo test --bin spreadsheet_engine
```

## Milestones

- [ ] Model sheets, cells, and typed cell values.
- [ ] Implement formula parser for references and arithmetic.
- [ ] Build dependency graph and recalculation scheduling.
- [ ] Detect circular references and propagate errors.
- [ ] Add import/export format for sample workbooks.
- [ ] Add tests for recalculation order, cycles, and formula correctness.

## Extra

- [ ] Add functions like SUM, AVG, IF, and range references.

## Tips

- Dependency tracking is the real core; UI can wait.
- Make formula errors explicit values instead of panics.
- Recalculation order should be deterministic for testability.
- Keep parser, evaluator, and graph maintenance separate.
