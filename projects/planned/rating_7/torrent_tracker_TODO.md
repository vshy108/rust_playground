# TODO: torrent_tracker (⭐ 7/10)

## Usage

```bash
cargo run --bin torrent_tracker
cargo test --bin torrent_tracker
```

## Milestones

- [ ] Implement announce request parsing and peer state model.
- [ ] Add swarm membership updates for start, stop, and completion events.
- [ ] Implement compact and normal peer list responses.
- [ ] Add interval calculation and peer eviction behavior.
- [ ] Implement scrape endpoint or aggregate swarm statistics.
- [ ] Add tests for peer lifecycle, response encoding, and stale-peer cleanup.

## Extra

- [ ] Add UDP tracker protocol support.

## Tips

- Peer identity and eviction rules define tracker correctness.
- Response encoding should be tested against fixtures, not eyeballed.
- Swarm statistics are derived state and should stay rebuildable.
- Keep tracker logic independent from any torrent-client assumptions.
