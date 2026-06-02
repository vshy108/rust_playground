// example must have main function
// Goal: Enterprise patterns

// Build:

// ```bash
// cargo run --bin kafka_consumer
// ```

// Learn:

// - async — each worker is a `tokio::spawn` task; tasks process messages concurrently
//   without blocking each other or the thread pool
// - worker pool — a fixed set of worker tasks pulls from a shared work channel; the pool
//   size caps parallelism without spawning an unbounded number of tasks

// Progress:

// Extra:

// - [ ] tracing — instrument each message with a span (`tracing::info_span!`) so retries
//   and DLQ moves are visible in structured logs

fn main() {
    println!("kafka_consumer: not yet implemented");
}
