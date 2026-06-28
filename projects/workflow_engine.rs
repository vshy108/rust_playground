// example must have main function
// Goal: State machines

// Build:

// ```bash
// cargo run --bin workflow_engine
// ```

// Learn:

// - graph — a workflow is a directed acyclic graph (DAG); nodes are steps, edges are
//   dependencies; represent with adjacency lists (`HashMap<NodeId, Vec<NodeId>>`)
// - execution model — topological sort determines a valid execution order; a node is
//   ready when all its predecessors have completed; use a queue of ready nodes

// Notes:

// Extra:

// - [ ] UI — a simple terminal visualization of node states (pending / running / done / failed)

fn main() {
    println!("workflow_engine: not yet implemented");
}
