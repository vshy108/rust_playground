# TODO: dependency_auditor (⭐ 7/10)

## Usage

```bash
cargo run --bin dependency_auditor
cargo test --bin dependency_auditor
```

## Milestones

- [ ] Build dependency graph from lock/manifests.
- [ ] Flag stale, vulnerable, or banned packages.
- [ ] Add policy checks for license and source trust.
- [ ] Generate actionable audit reports.
- [ ] Add tests for graph traversal and rule matching.

## Extra

- [ ] Add pull-request friendly report output.

## Tips

- Treat package identity as ecosystem plus name plus version.
- Keep data collection and policy evaluation decoupled.
