# TODO: service_mesh_sidecar (⭐ 9/10)


## Usage

```bash
cargo run --bin sidecar -- --upstream 127.0.0.1:8080 --port 9090
cargo test --bin sidecar
```

## 1. TCP transparent proxy

- [ ] `TcpListener::bind` on the listen port.
- [ ] For each accepted connection, open a connection to the upstream.
- [ ] Use `tokio::io::copy` in both directions concurrently (`tokio::join!`).

Acceptance check: `curl localhost:9090` proxies to the upstream and returns its response.

## 2. Byte counters

- [ ] Track `bytes_in` and `bytes_out` per proxied connection using `AtomicU64`.
- [ ] Accumulate global totals across all connections.

Acceptance check: totals increment correctly after a known-size request.

## 3. Latency measurement

- [ ] Record connection duration from accept to close.
- [ ] Maintain a running count and sum for mean latency.

Acceptance check: mean latency is non-zero after one proxied connection.

## 4. Error tracking

- [ ] Count failed upstream connections (refused, timeout).
- [ ] Expose the error count alongside byte/latency metrics.

Acceptance check: pointing the sidecar at a closed port increments the error count.

## 5. Metrics endpoint

- [ ] Bind a second `TcpListener` on a metrics port (default 9091).
- [ ] Respond to `GET /metrics` with a plain-text snapshot of all counters.

Acceptance check: `curl localhost:9091/metrics` prints bytes, latency, and error count.

## 6. Tests

- [ ] Proxy passes bytes unchanged.
- [ ] Byte counter matches payload size.
- [ ] Failed upstream increments error counter.

## Extra: routing

- [ ] Read the first line of each incoming HTTP request; match `Host` header to a route table;
  forward to different upstreams per host.

## Tips

- Start with protocol and contract tests first (request, response, error, timeout).
- Build a strict parser before adding convenience behavior; fail closed on malformed input.
- Add structured request logging early so debugging network paths is cheap.
- Keep connection lifecycle explicit: open, active, idle timeout, close.
- Add load and latency checks after correctness is stable.

## Learn Notes

- TCP — `TcpListener` accepts connections; `TcpStream` is a bidirectional byte pipe; `tokio::io::copy` pumps bytes between two streams concurrently
- observability — count bytes in/out, request latency, and error count per upstream; expose a `/metrics` endpoint for scraping

## Extra

- routing — inspect the first request line to route HTTP traffic to different upstreams

