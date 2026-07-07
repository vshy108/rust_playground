# TODO: dist_cache (⭐ 10/10)


## Usage

```bash
# Terminal 1
cargo run --bin dist_cache -- --node 127.0.0.1:7001 --peers 127.0.0.1:7002

# Terminal 2
cargo run --bin dist_cache -- --node 127.0.0.1:7002 --peers 127.0.0.1:7001

cargo test --bin dist_cache
```

## 1. Single-node in-memory cache

- [ ] Build a `NodeCache` with `get(key)` / `put(key, value)` backed by a `HashMap`.
- [ ] Expose over TCP: `SET key value\n` → `OK\n`; `GET key\n` → `value\n` or `NIL\n`.

Acceptance check: single-node SET/GET round-trip works.

## 2. Consistent hashing ring

- [ ] Implement a hash ring: each node owns a range of the key space.
- [ ] `responsible_node(key) -> NodeAddr` returns the node that owns a key.
- [ ] Adding/removing a node remaps only adjacent keys.

Acceptance check: 1000 keys distribute roughly evenly across 3 nodes.

## 3. Node join / peer discovery

- [ ] On startup, connect to each `--peer`; exchange node lists.
- [ ] Update the ring when a new peer announces itself.

Acceptance check: node 1 and node 2 both route a key to the same node after joining.

## 4. Replication

- [ ] On `PUT`, forward the write to the next N-1 nodes on the ring (replication factor N=2).
- [ ] A `PUT` succeeds when at least W=1 replica acknowledges.

Acceptance check: a key written to node 1 is readable from node 2.

## 5. Failover

- [ ] Detect an unresponsive peer (connection timeout).
- [ ] Re-route reads to the next live replica on the ring.

Acceptance check: stopping node 2 does not prevent reading its keys via node 1.

## 6. Tests

- [ ] Hash ring distributes keys deterministically.
- [ ] Replication write reaches the replica.
- [ ] Failover routes around a dead node.

## Extra: partition tolerance

- [ ] Track a quorum counter; refuse writes when fewer than W nodes are reachable.
- [ ] Return a clear error to the client instead of silently succeeding on one replica.

## Tips

- Lock down data invariants first and encode them in tests.
- Implement persistence boundaries early (snapshot, log, recovery) even if minimal.
- Separate correctness path from optimization path; optimize only after passing invariants.
- Add deterministic simulation tests for retries, crashes, and restart behavior.
- Track state transitions with trace logs to simplify post-failure analysis.

## Learn Notes

- consensus — a quorum write (W > N/2) ensures at least one replica has the data before acknowledging; a quorum read (R > N/2) guarantees reading a node that saw the write
- distributed systems — consistent hashing assigns keys to nodes; adding/removing a node remaps only a fraction of keys; vector clocks or last-write-wins resolve write conflicts

## Extra

- partition tolerance — detect split-brain; refuse writes when quorum is unavailable

