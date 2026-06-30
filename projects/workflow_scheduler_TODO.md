# TODO: workflow_scheduler (⭐ 7/10)

## Usage

```bash
cargo run --bin workflow_scheduler
cargo test --bin workflow_scheduler
```

## Milestones

- [ ] Model scheduled workflows, triggers, and execution state.
- [ ] Implement cron-like or fixed-interval trigger parsing.
- [ ] Add durable queueing of due executions.
- [ ] Implement retry policy and backoff per scheduled job.
- [ ] Add pause/resume and missed-run handling semantics.
- [ ] Add tests for trigger calculation, retries, and restart recovery.

## Extra

- [ ] Add dependency-aware scheduling between workflows.

## Tips

- Time calculations should be isolated and testable.
- Persist next-run state explicitly; recomputation after restart can drift.
- Define missed-run semantics early: catch up, skip, or coalesce.
- Keep trigger parsing separate from execution logic.
