// example must have main function
// Goal: Infra

// Build:

// ```bash
// cargo run --bin otel_collector
// ```

// Learn:

// - streaming — receive telemetry over a socket (OTLP/gRPC or HTTP); process records
//   as they arrive without buffering the full stream in memory
// - batching — accumulate spans/metrics in a buffer; flush when the buffer reaches a
//   size threshold or a time deadline, whichever comes first

// Progress:

// Extra:

// - [ ] metrics + traces — handle both signal types in the same pipeline

fn main() {
    println!("otel_collector: not yet implemented");
}
