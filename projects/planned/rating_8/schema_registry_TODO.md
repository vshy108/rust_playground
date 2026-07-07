# TODO: schema_registry (⭐ 8/10)

## Usage

```bash
cargo run --bin schema_registry
cargo test --bin schema_registry
```

## Milestones

- [ ] Implement schema subjects, versions, and compatibility modes.
- [ ] Add Avro, JSON Schema, or Protobuf descriptor storage model.
- [ ] Implement schema registration with compatibility validation.
- [ ] Add lookup APIs by subject, version, and global identifier.
- [ ] Implement soft delete, latest-version resolution, and audit history.
- [ ] Add tests for compatibility checks, version ordering, and descriptor round-trips.

## Extra

- [ ] Add client-side caching and producer/consumer integration examples.

## Tips

- Start by modeling schema metadata before supporting wire formats.
- Compatibility rules are the core behavior; isolate them from transport code.
- Global IDs and subject versions are different concerns and should stay separate.
- Fixture-driven compatibility tests will keep rule changes honest.
