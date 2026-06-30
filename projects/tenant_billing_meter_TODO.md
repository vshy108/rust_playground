# TODO: tenant_billing_meter (⭐ 6/10)

## Usage

```bash
cargo run --bin tenant_billing_meter
cargo test --bin tenant_billing_meter
```

## Milestones

- [ ] Define billable usage events and units.
- [ ] Aggregate usage by tenant and billing period.
- [ ] Apply pricing rules and tiered thresholds.
- [ ] Emit invoice-ready summaries.
- [ ] Add tests for rounding, tiers, and rollovers.

## Extra

- [ ] Add forecasting for end-of-period usage.

## Tips

- Keep raw usage events immutable for auditability.
- Separate pricing rules from metering aggregation.
