// example must have main function
// Goal: Production backend

// Build:

// ```bash
// cargo run --bin rest_api
// ```

// Learn:

// - async — `async fn` returns a Future; `.await` suspends the task without blocking the thread
// - middleware — tower layers applied to every request (logging, auth, error handling)
// - axum routing — `Router::new().route(path, method(handler))` wires HTTP methods to handlers
// - serde — `#[derive(Deserialize, Serialize)]` on structs for JSON request/response bodies
// - State — `Arc<Mutex<T>>` shared across handlers; cloned into each via axum's `State` extractor

// Progress:

// Extra:

// - [ ] JWT authentication middleware

fn main() {
    println!("rest_api: not yet implemented");
}
