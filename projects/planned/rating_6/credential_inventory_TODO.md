# TODO: credential_inventory (⭐ 6/10)

## Usage

```bash
cargo run --bin credential_inventory
cargo test --bin credential_inventory
```

## Milestones

- [ ] Define credential metadata and ownership records.
- [ ] Ingest credentials from multiple source systems.
- [ ] Detect stale, orphaned, or weakly-scoped credentials.
- [ ] Produce remediation queue outputs.
- [ ] Add tests for ownership and stale-detection rules.

## Extra

- [ ] Add cryptoperiod policy checks.

## Tips

- Never store raw secret values; only inventory metadata.
- Keep source adapters isolated for safer integration testing.
