// example must have main function
// Goal: Rust architect level

// Build:

// ```bash
// cargo run --bin dist_cache -- --node 127.0.0.1:7001 --peers 127.0.0.1:7002,127.0.0.1:7003
// ```

// Learn:

// - consensus — a quorum write (W > N/2) ensures at least one replica has the data before
//   acknowledging; a quorum read (R > N/2) guarantees reading a node that saw the write
// - distributed systems — consistent hashing assigns keys to nodes; adding/removing a node
//   remaps only a fraction of keys; vector clocks or last-write-wins resolve write conflicts

// Progress:

// Extra:

// - [ ] partition tolerance — detect split-brain; refuse writes when quorum is unavailable

fn main() {
    println!("dist_cache: not yet implemented");
}
