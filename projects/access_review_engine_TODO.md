# TODO: access_review_engine (⭐ 7/10)

## Usage

```bash
cargo run --bin access_review_engine
cargo test --bin access_review_engine
```

## Milestones

- [ ] Model principals, roles, and resource grants.
- [ ] Generate periodic review campaigns by policy.
- [ ] Capture approve/revoke decisions with evidence.
- [ ] Enforce overdue escalation and reminders.
- [ ] Add tests for entitlement expansion and revocation rules.

## Extra

- [ ] Add risk-based prioritization for high-privilege access.

## Tips

- Keep entitlement graph traversal deterministic for audits.
- Persist review evidence immutably to preserve traceability.
