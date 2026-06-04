// example must have main function
// Goal: Observability

// Build:

// ```bash
// cargo run --bin metrics_collector
// ```

// Learn:

// - channels — `std::sync::mpsc` (or `tokio::sync::mpsc`) decouples metric producers from
//   the aggregator; producers send `MetricEvent` values; the aggregator owns the receiver
// - aggregation — the aggregator loop accumulates counters and histograms in a HashMap;
//   a query interface reads snapshots without blocking producers

// Notes:

// Extra:

// - [ ] Prometheus endpoint — expose `/metrics` in the text-based Prometheus exposition format

fn main() {
    println!("metrics_collector: not yet implemented");
}
