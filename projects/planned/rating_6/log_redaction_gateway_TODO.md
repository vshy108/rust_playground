# TODO: log_redaction_gateway (⭐ 6/10)

## Usage

```bash
cargo run --bin log_redaction_gateway
cargo test --bin log_redaction_gateway
```

## Milestones

- [ ] Parse structured and line-based log events.
- [ ] Apply redaction rules for secrets and PII.
- [ ] Preserve trace context while masking sensitive fields.
- [ ] Emit redaction metrics and false-positive counters.
- [ ] Add tests for redaction correctness and passthrough safety.

## Extra

- [ ] Add rule simulation mode over captured log files.

## Tips

- Keep redaction rules order-aware to avoid accidental leaks.
- Prefer deterministic masking for repeatable tests.
