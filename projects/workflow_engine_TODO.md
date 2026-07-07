# TODO: workflow_engine (⭐ 8/10)


## Usage

```bash
cargo run --bin workflow_engine
cargo test --bin workflow_engine
```

## 1. Graph model

- [ ] Define `NodeId = usize`.
- [ ] Define a `Step` struct: `id: NodeId`, `name: String`, `deps: Vec<NodeId>`.
- [ ] Define a `Workflow` struct: `steps: HashMap<NodeId, Step>`.
- [ ] Write `Workflow::add_step` and `Workflow::add_dependency`.

Acceptance check: a 3-node chain (A → B → C) is representable without errors.

## 2. Topological sort

- [ ] Implement Kahn's algorithm: start with nodes that have no in-edges; peel off one
  layer at a time; detect cycles (remaining nodes after sort = cycle).

Acceptance check: A→B→C sorts to `[A, B, C]`; a cycle returns an error.

## 3. Execution engine

- [ ] Define `NodeState` enum: `Pending`, `Running`, `Done`, `Failed(String)`.
- [ ] Maintain a `HashMap<NodeId, NodeState>`.
- [ ] Drive execution: ready queue = nodes whose deps are all `Done`; execute each;
  mark `Done` or `Failed`.

Acceptance check: executing A→B→C calls each step in order; a failed A stops B and C.

## 4. Async execution

- [ ] Replace synchronous step execution with `async fn`; spawn each ready node as a task.
- [ ] Wait for all tasks in a batch to complete before advancing.

Acceptance check: independent nodes (A→C, B→C) run concurrently; C runs after both complete.

## 5. Tests

- [ ] Topological sort of a simple chain.
- [ ] Cycle detection returns error.
- [ ] Workflow with a failing node stops dependents.
- [ ] Independent nodes execute concurrently.

## Extra: terminal UI

- [ ] Print a live status table (node name, state) using ANSI escape codes or `crossterm`.

## Tips

- Lock down data invariants first and encode them in tests.
- Implement persistence boundaries early (snapshot, log, recovery) even if minimal.
- Separate correctness path from optimization path; optimize only after passing invariants.
- Add deterministic simulation tests for retries, crashes, and restart behavior.
- Track state transitions with trace logs to simplify post-failure analysis.

## Learn Notes

- graph — a workflow is a directed acyclic graph (DAG); nodes are steps, edges are dependencies; represent with adjacency lists (`HashMap<NodeId, Vec<NodeId>>`)
- execution model — topological sort determines a valid execution order; a node is ready when all its predecessors have completed; use a queue of ready nodes

## Extra

- UI — a simple terminal visualization of node states (pending / running / done / failed)

