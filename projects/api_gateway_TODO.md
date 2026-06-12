# TODO: api_gateway

## Usage

```bash
cargo run --bin api_gateway
cargo test --bin api_gateway
```

---

# Completed

## Routing

* [x] Define `Route` configuration.
* [x] Longest-prefix route matching.
* [x] Route specificity resolution.
* [x] Parameterized routes (`/users/:id`).
* [x] Root catch-all route (`/`).
* [x] Route matching unit tests.

Acceptance check:

* `/users/admin` beats `/users/:id`
* `/users/admin/1` matches admin route
* unknown paths fall back to `/`

---

## Reverse Proxy

* [x] Accept incoming Axum requests.
* [x] Forward method, URI, query string, headers, and body to upstream.
* [x] Forward upstream response status, headers, and body.
* [x] Return `404 Not Found` when no route matches.
* [x] Return `502 Bad Gateway` when upstream is unreachable.

Acceptance check:

* Local echo server receives original request.
* Gateway returns upstream response unchanged.

---

## Header Filtering

* [x] Remove hop-by-hop request headers.
* [x] Remove:

  * `Connection`
  * `Keep-Alive`
  * `Proxy-Authenticate`
  * `Proxy-Authorization`
  * `TE`
  * `Trailer`
  * `Transfer-Encoding`
  * `Upgrade`

Acceptance check:

* Upstream never receives hop-by-hop headers.

---

## Middleware

* [x] Authentication middleware.
* [x] Request timeout middleware.
* [x] Rate limiting middleware.
* [x] Request ID generation.
* [x] Request ID propagation.
* [x] Request logging.

Acceptance check:

* Missing Authorization header returns `401`.
* Request IDs are preserved or generated automatically.
* Slow requests return timeout response.

---

## Platform

* [x] Health endpoint (`/health`)
* [x] Readiness endpoint (`/ready`)
* [x] Graceful shutdown (`Ctrl+C`)

Acceptance check:

* Health and readiness return `200`.
* Gateway shuts down cleanly.

---

## Integration Tests

* [x] Route matching tests.
* [x] Proxy forwarding tests.
* [x] Upstream failure tests.
* [x] Header filtering tests.
* [x] Authentication tests.
* [x] Timeout tests.
* [x] Request ID tests.

---

# Next Features

## Retry Middleware

* [ ] Retry failed upstream requests.
* [ ] Retry on:

  * connection failures
  * timeouts
  * configurable 5xx responses
* [ ] Exponential backoff.

Acceptance check:

* Upstream fails once then succeeds.
* Gateway eventually returns success.

---

## Load Balancing

* [ ] Multiple upstream instances per route.
* [ ] Round-robin strategy.
* [ ] Health-aware instance selection.

Acceptance check:

* Requests are distributed across multiple upstreams.

---

## Circuit Breaker

* [ ] Track upstream failures.
* [ ] Open circuit after threshold.
* [ ] Half-open recovery mode.

Acceptance check:

* Failed upstream is temporarily bypassed.

---

## Service Discovery

* [ ] Static route reload.
* [ ] Eureka integration.
* [ ] Consul integration.
* [ ] Kubernetes service discovery.

Acceptance check:

* New upstreams appear without restart.

---

## OpenTelemetry

* [ ] Add `opentelemetry`.
* [ ] Add `tracing-opentelemetry`.
* [ ] Propagate W3C Trace Context.
* [ ] Export traces to Jaeger or Tempo.

Acceptance check:

* Single trace visible across gateway and upstream.

---

## Metrics

* [ ] Prometheus metrics endpoint.
* [ ] Request counters.
* [ ] Response status counters.
* [ ] Latency histograms.

Acceptance check:

* Metrics visible from Prometheus scrape.

---

## Authentication

* [ ] JWT validation.
* [ ] Claims extraction.
* [ ] Role-based authorization.

Acceptance check:

* Invalid JWT returns `401`.
* Missing role returns `403`.

---

## Dynamic Configuration

* [ ] Load routes from file.
* [ ] Hot reload route table.
* [ ] Validate configuration before apply.

Acceptance check:

* Route changes take effect without restart.

---

## Caching

* [ ] Response cache.
* [ ] Cache TTL configuration.
* [ ] Cache invalidation.

Acceptance check:

* Repeated requests are served from cache.
