# TODO: api_contract_tester (⭐ 6/10)

## Usage

```bash
cargo run --bin api_contract_tester
cargo test --bin api_contract_tester
```

## Milestones

- [ ] Model endpoint contracts with request/response assertions.
- [ ] Execute contract suites against target environments.
- [ ] Add schema and status-code validation checks.
- [ ] Produce human-readable failure diff reports.
- [ ] Add tests for matcher behavior and report output.

## Extra

- [ ] Add snapshot-based contract baselining.

## Tips

- Keep matcher primitives composable for reuse across endpoints.
- Separate execution transport from assertion logic.
