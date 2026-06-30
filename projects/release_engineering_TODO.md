# TODO: release_engineering (⭐ 7/10)

## Usage

```bash
cargo run --bin release_engineering
cargo test --bin release_engineering
```

## Milestones

- [ ] Parse release configuration and versioning rules.
- [ ] Implement changelog generation from commit or issue metadata.
- [ ] Add artifact build matrix modeling.
- [ ] Implement release candidate and final release promotion flow.
- [ ] Add signing/checksum manifest generation.
- [ ] Add tests for semver bump logic, changelog output, and artifact manifest correctness.

## Extra

- [ ] Add Git hosting integration for release publishing.

## Tips

- Keep version computation deterministic and isolated.
- Changelog generation should be reproducible from the same git state.
- Artifact metadata is as important as the binaries themselves.
- Model release states explicitly to avoid half-published flows.
