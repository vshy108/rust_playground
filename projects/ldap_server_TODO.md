# TODO: ldap_server (⭐ 8/10)

## Usage

```bash
cargo run --bin ldap_server
cargo test --bin ldap_server
```

## Milestones

- [ ] Implement LDAP entry, DN, and attribute schema model.
- [ ] Add bind, search, add, modify, and delete operation handling.
- [ ] Implement filter parsing and subtree/base search semantics.
- [ ] Add access control and password storage flow.
- [ ] Implement pagination or size/time limit behavior.
- [ ] Add tests for DN normalization, filter matching, and auth failures.

## Extra

- [ ] Add replication or change notification support.

## Tips

- DN normalization rules should be explicit and fixture-tested.
- Search filter parsing is a natural boundary from storage execution.
- Auth and directory mutation should not be coupled to transport code.
- Entry schema constraints matter even in a learning implementation.
