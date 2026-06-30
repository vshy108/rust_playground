# TODO: log_parser (⭐ 4/10)

## Usage

```bash
cargo run --bin logparse -- fixtures/access.log
cargo test --bin logparse
```

## 1. Data model

- [x] Define a `LogEntry` struct: IP, method, path, status code, latency (ms).
- [x] Write a `parse_line(line: &str) -> Option<LogEntry>` function.

Acceptance check: `parse_line` returns `Some` for a valid line, `None` for malformed input.

## 2. File reading

- [x] Read the log file path from CLI args.
- [x] Read the file into lines; skip unparseable lines with a warning count.

Acceptance check: running against a sample log prints line count.

## 3. Top IPs

- [x] Count requests per IP using `HashMap<IpAddr, usize>`.
- [x] Print the top 5 IPs by request count.

Acceptance check: top IP matches manual count in sample.

## 4. Latency stats

- [x] Compute mean and p99 latency across all entries.

Acceptance check: mean and p99 print for sample file.

## 5. Error rate

- [x] Count 5xx responses; compute error rate = errors / total.

Acceptance check: error rate prints for sample file.

## 6. Tests

- [x] `parse_line` round-trips a known log line.
- [x] Top-IP aggregation over a small in-memory slice.
- [x] Error rate calculation over a fixed set of entries.

## Extra: CSV export

- [x] Add `--csv out.csv` flag; write one row per IP with request count and mean latency.
- [x] `ip_stats` aggregation: count + mean latency per IP in one pass.
- [x] `write_csv` writes header + one row per IP with 2-decimal mean latency.
- [x] Tests: `ip_stats_returns_count_and_mean_latency`, `write_csv_produces_correct_rows`.

## Tips

- Implement one milestone at a time and keep each slice testable.
- Add a failing test first, then implement the smallest behavior to pass.
- Keep CLI/API surface stable while iterating internals.
- Validate both happy path and error path for every milestone.
- Run focused tests before broad checks.
