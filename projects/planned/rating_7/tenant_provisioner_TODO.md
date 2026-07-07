# TODO: tenant_provisioner (⭐ 7/10)

## Usage

```bash
cargo run --bin tenant_provisioner
cargo test --bin tenant_provisioner
```

## Milestones

- [ ] Model tenant lifecycle states and required resources.
- [ ] Implement idempotent provisioning workflow.
- [ ] Add rollback/cleanup for partial failures.
- [ ] Add status tracking and event emission.
- [ ] Add tests for retries, idempotency, and rollback.

## Extra

- [ ] Add per-tenant policy templates.

## Tips

- Persist workflow checkpoints so resumes are safe after restarts.
- Keep resource adapters behind traits for testability.
