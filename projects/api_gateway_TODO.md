# TODO: api_gateway

## Usage

```bash
cargo run --bin api_gateway
cargo test --bin api_gateway
```

## 1. Route table

- [ ] Define a `Route` struct: `prefix: String`, `upstream: String`.
- [ ] Write a `match_route(path: &str, routes: &[Route]) -> Option<&Route>` function.

Acceptance check: longest-prefix match returns the correct upstream for known paths.

## 2. Reverse proxy handler

- [ ] Accept an incoming `axum` request.
- [ ] Forward it to the matched upstream using `reqwest`; stream the response back.
- [ ] Return 502 when the upstream is unreachable; 404 when no route matches.

Acceptance check: proxying to a local echo server returns the original body.

## 3. Timeout middleware

- [ ] Wrap the upstream call with `tokio::time::timeout`.
- [ ] Return 504 Gateway Timeout when the deadline is exceeded.

Acceptance check: a slow upstream (sleep > timeout) returns 504.

## 4. Retry middleware

- [ ] Retry the upstream call up to N times on 5xx or connection error.
- [ ] Apply exponential back-off between retries.

Acceptance check: a flaky upstream (fails once then succeeds) returns 200 after one retry.

## 5. Rate limiting

- [ ] Track request counts per client IP in a sliding window.
- [ ] Return 429 Too Many Requests when the limit is exceeded.

Acceptance check: sending 11 requests with a limit of 10 returns one 429.

## 6. Tests

- [ ] Route matching for known and unknown paths.
- [ ] Timeout fires on a slow upstream mock.
- [ ] Rate limit returns 429 after threshold.

## Extra: OpenTelemetry

- [ ] Add `opentelemetry` + `tracing-opentelemetry`; propagate W3C Trace Context headers.
