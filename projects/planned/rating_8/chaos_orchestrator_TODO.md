# TODO: chaos_orchestrator (⭐ 8/10)

## Usage

```bash
cargo run --bin chaos_orchestrator
cargo test --bin chaos_orchestrator
```

## Milestones

- [ ] Model fault experiments and blast-radius constraints.
- [ ] Implement experiment scheduling and lifecycle controls.
- [ ] Add fault injectors (latency, drop, process restart).
- [ ] Add stop conditions and safety guardrails.
- [ ] Add tests for experiment safety and rollback behavior.

## Extra

- [ ] Add canary-only experiment targeting.

## Tips

- Keep fault definitions declarative to make reviews and audits easier.
- Separate planner from injectors so execution adapters stay replaceable.
