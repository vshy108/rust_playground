# TODO: uptime_sla_tracker (⭐ 5/10)

## Usage

```bash
cargo run --bin uptime_sla_tracker
cargo test --bin uptime_sla_tracker
```

## Milestones

- [ ] Define service-level objectives and error budgets.
- [ ] Ingest uptime events and outage intervals.
- [ ] Compute rolling SLA compliance windows.
- [ ] Generate breach warnings and monthly summaries.
- [ ] Add tests for boundary windows and downtime overlap.

## Extra

- [ ] Add burn-rate style alerts.

## Tips

- Normalize event timestamps before window calculations.
- Keep objective definitions immutable per reporting period.
