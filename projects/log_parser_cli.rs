// example must have main function
// Goal: Real backend work

// Build:

// ```bash
// cargo run --bin logparse -- access.log
// ```

// Learn:

// - iterators — chaining map/filter/fold to aggregate log data without intermediate allocations
// - parsing — splitting lines into fields; using split_once and FromStr for typed values
// - aggregation — HashMap counters for per-IP counts; running totals for latency and errors

// Progress:

// Extra:

// - [ ] CSV export

fn main() {
    println!("log_parser_cli: not yet implemented");
}
