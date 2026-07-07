# TODO: service_catalog (⭐ 6/10)

## Usage

```bash
cargo run --bin service_catalog
cargo test --bin service_catalog
```

## Milestones

- [ ] Model service metadata, owners, and runtime environments.
- [ ] Add registration and update APIs with validation.
- [ ] Support dependency mapping between services.
- [ ] Add search/filter endpoints for catalog queries.
- [ ] Add tests for schema validation and relationship integrity.

## Extra

- [ ] Add ownership escalation chain views.

## Tips

- Keep service identity stable and immutable once created.
- Separate metadata storage from query indexing concerns.
