// example must have main function
// Goal: Architecture

// Build:

// ```bash
// cargo run --bin api_gateway
// ```

// Learn:

// - middleware — tower `Layer`/`Service` traits; each layer wraps the inner service and can
//   intercept requests and responses (logging, auth, rate limiting) without changing handlers
// - resilience — retry on transient errors with exponential back-off; timeout cancels a slow
//   upstream; rate limiting caps requests per time window per client

// Progress:

// Extra:

// - [ ] OpenTelemetry tracing — propagate trace context to upstream services

fn main() {
    println!("api_gateway: not yet implemented");
}
