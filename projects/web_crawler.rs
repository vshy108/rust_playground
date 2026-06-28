// example must have main function
// Goal: Async mindset

// Build:

// ```bash
// cargo run --bin crawler -- https://example.com
// ```

// Learn:

// - futures — a Future is a lazy computation; it does nothing until polled by an executor
// - task scheduling — `tokio::spawn` creates an independent task; tasks run concurrently on
//   the tokio thread pool; `JoinHandle` lets the spawner await the result
// - async channels — `tokio::sync::mpsc` passes URLs between the discovery task and workers
//   without blocking; the channel decouples producers from consumers

// Notes:

// Extra:

// - [ ] limit concurrency with a semaphore (`tokio::sync::Semaphore`)

fn main() {
    println!("web_crawler: not yet implemented");
}
