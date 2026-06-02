// example must have main function
// Goal: Networking mastery

// Build:

// ```bash
// cargo run --bin sidecar -- --upstream 127.0.0.1:8080 --port 9090
// ```

// Learn:

// - TCP — `TcpListener` accepts connections; `TcpStream` is a bidirectional byte pipe;
//   `tokio::io::copy` pumps bytes between two streams concurrently
// - observability — count bytes in/out, request latency, and error count per upstream;
//   expose a `/metrics` endpoint for scraping

// Progress:

// Extra:

// - [ ] routing — inspect the first request line to route HTTP traffic to different upstreams

fn main() {
    println!("service_mesh_sidecar: not yet implemented");
}
