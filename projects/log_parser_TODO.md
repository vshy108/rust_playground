# TODO: log_parser_cli

## Usage

```bash
cargo run --bin logparse -- fixtures/access.log
cargo test --bin logparse
```

## 1. Data model

- [ ] Define a `LogEntry` struct: IP, method, path, status code, latency (ms).
- [ ] Write a `parse_line(line: &str) -> Option<LogEntry>` function.

Acceptance check: `parse_line` returns `Some` for a valid line, `None` for malformed input.

## 2. File reading

- [ ] Read the log file path from CLI args.
- [ ] Read the file into lines; skip unparseable lines with a warning count.

Acceptance check: running against a sample log prints line count.

## 3. Top IPs

- [ ] Count requests per IP using `HashMap<String, usize>`.
- [ ] Print the top 5 IPs by request count.

Acceptance check: top IP matches manual count in sample.

## 4. Latency stats

- [ ] Compute mean and p99 latency across all entries.

Acceptance check: mean and p99 print for sample file.

## 5. Error rate

- [ ] Count 5xx responses; compute error rate = errors / total.

Acceptance check: error rate prints for sample file.

## 6. Tests

- [ ] `parse_line` round-trips a known log line.
- [ ] Top-IP aggregation over a small in-memory slice.
- [ ] Error rate calculation over a fixed set of entries.

## Extra: CSV export

- [ ] Add `--csv out.csv` flag; write one row per IP with request count and mean latency.
