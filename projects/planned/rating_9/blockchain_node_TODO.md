# TODO: blockchain_node (⭐ 9/10)

## Usage

```bash
cargo run --bin blockchain_node
cargo test --bin blockchain_node
```

## Milestones

- [ ] Implement block and transaction data structures with validation.
- [ ] Add mempool and block assembly workflow.
- [ ] Implement peer protocol for block and transaction gossip.
- [ ] Add consensus rule checks for chain extension and fork handling.
- [ ] Implement persistent chain state and restart recovery.
- [ ] Add tests for transaction validity, fork choice, and replay safety.

## Extra

- [ ] Add light-client proof API for simplified verification.

## Tips

- Keep consensus rules deterministic and side-effect free where possible.
- Validate every block transition against explicit state preconditions.
- Persist chain metadata atomically to avoid corruption on crash.
- Build adversarial fixtures for malformed blocks and network partitions.
