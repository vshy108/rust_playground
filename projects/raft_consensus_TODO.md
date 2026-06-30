# TODO: raft_consensus (⭐ 10/10)

## Usage

```bash
cargo run --bin raft_consensus
cargo test --bin raft_consensus
```

## Milestones

- [ ] Implement node state machine (Follower, Candidate, Leader).
- [ ] Add RequestVote and AppendEntries RPC encoding/decoding.
- [ ] Implement leader election with randomized timeouts.
- [ ] Add log replication and commit index advancement rules.
- [ ] Add durable term/vote/log persistence layer abstraction.
- [ ] Add tests for election safety and log matching property.

## Extra

- [ ] Add membership change via joint consensus workflow.

## Tips

- Lock down data invariants first and encode them in tests.
- Implement persistence boundaries early (snapshot, log, recovery) even if minimal.
- Separate correctness path from optimization path; optimize only after passing invariants.
- Add deterministic simulation tests for retries, crashes, and restart behavior.
- Track state transitions with trace logs to simplify post-failure analysis.
