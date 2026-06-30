# TODO: api_gateway (⭐ 7/10)

---

````md
# TODO: api_gateway

## Usage

```bash
cargo run --bin api_gateway
cargo test --bin api_gateway
````

---

# Completed

## Routing

* [x] Define `Route` configuration
* [x] Longest-prefix route matching
* [x] Route specificity resolution (depth + static priority)
* [x] Parameterized routes (`/users/:id`)
* [x] Root catch-all route (`/`)
* [x] Route matching unit tests

### Acceptance

* `/users/admin` beats `/users/:id`
* `/users/admin/1` matches admin route
* unknown paths fall back to `/`

---

## Reverse Proxy

* [x] Axum request ingestion
* [x] Forward method, URI, query string, headers, body
* [x] Stream upstream request (no buffering)
* [x] Forward upstream response status, headers, body
* [x] Return `404 Not Found` when no route matches
* [x] Return `502 Bad Gateway` on upstream failure
* [x] Return `504 Gateway Timeout` via timeout middleware

### Acceptance

* Local upstream receives identical request
* Response is transparently proxied
* Failure modes correctly mapped to HTTP status codes

---

## Header Filtering

* [x] Hop-by-hop header removal (request side)
* [x] Hop-by-hop header removal (response side)
* [x] Preserve safe headers only

### Removed headers

* Connection
* Keep-Alive
* Proxy-Authenticate
* Proxy-Authorization
* TE
* Trailer
* Transfer-Encoding
* Upgrade

### Acceptance

* Upstream never receives hop-by-hop headers

---

## Middleware Stack

* [x] Authentication middleware (header-based)
* [x] Request timeout middleware (Tower TimeoutLayer)
* [x] Rate limiting (tower_governor, IP-based)
* [x] Request ID generation
* [x] Request ID propagation
* [x] Structured logging (TraceLayer)
* [x] Graceful shutdown (Ctrl+C signal)

### Acceptance

* Missing Authorization → `401`
* Requests exceed timeout → `504`
* Request ID consistent across gateway + upstream logs

---

## Platform Endpoints

* [x] `/health`
* [x] `/ready`
* [x] Graceful shutdown

### Acceptance

* Health returns `200 OK`
* Ready returns `200 OK`
* Ctrl+C stops server cleanly

---

## Load Balancing

* [x] Round-robin upstream selection
* [x] Shared atomic counter across requests

### Acceptance

* Multiple upstreams receive distributed traffic
* Sequence is deterministic per process

---

## Integration Tests

* [x] Route matching
* [x] Proxy forwarding
* [x] Hop-by-hop header filtering
* [x] Authentication middleware
* [x] Timeout handling
* [x] Request ID propagation
* [x] Round-robin distribution
* [x] Upstream failure handling

---

# Next Features

## Retry Policy

* [ ] Retry upstream requests on failure
* [ ] Retry conditions:
  * connection failure
  * timeout
  * selected 5xx responses
* [ ] Exponential backoff with jitter

### Acceptance

* Failed upstream automatically retried once or more
* Eventually succeeds if upstream recovers

---

## Circuit Breaker

* [ ] Track failure rate per upstream
* [ ] Open state after threshold
* [ ] Half-open recovery probing
* [ ] Per-route isolation

### Acceptance

* Faulty upstream excluded temporarily
* Recovery restores traffic automatically

---

## Service Discovery

* [ ] Static route config (current baseline)
* [ ] File-based reload support
* [ ] Kubernetes service discovery (future)
* [ ] Consul / Eureka adapters (optional)

### Acceptance

* Upstream list updates without binary restart

---

## Observability (OpenTelemetry)

* [ ] Distributed tracing integration
* [ ] W3C Trace Context propagation
* [ ] Span per request lifecycle
* [ ] Export to Jaeger / Tempo

### Acceptance

* End-to-end trace visible across gateway + upstream

---

## Metrics

* [ ] Prometheus metrics endpoint
* [ ] Request counter
* [ ] Error counter
* [ ] Latency histogram
* [ ] Upstream health metrics

### Acceptance

* Metrics scrapeable from `/metrics`

---

## Security Enhancements

* [ ] JWT validation middleware
* [ ] Role-based authorization
* [ ] Claims propagation to upstream
* [ ] Optional mTLS upstream communication

### Acceptance

* Invalid tokens rejected (401)
* Unauthorized roles rejected (403)

---

## Caching Layer

* [ ] Response caching (in-memory)
* [ ] Cache key based on method + path + query
* [ ] TTL-based eviction

### Acceptance

* Repeated requests return cached response

Below is a **clean production-grade split view**:

1. ✅ What you already have (with weaknesses clearly called out)
2. 🚧 Industry-grade TODO (what real gateways like Envoy / Kong / NGINX would push next)

No fluff — only architectural gaps that matter.

---

# 🧱 Current Implementation — Reality Review

## 1. Routing Layer

### ✅ What you already did well

* Prefix-based routing
* Param routes (`/users/:id`)
* Deterministic scoring (depth + static priority)
* Root fallback

### ⚠️ Weak points

* ❌ No route caching (O(n) match per request)
* ❌ No compiled routing tree (still linear scan style)
* ❌ No regex or constraint-based routing
* ❌ No per-route middleware stack
* ❌ No route metadata (auth policy, timeout per route, retries)

👉 Production gap:

> Current design behaves like “config list router”, not “routing engine”

---

## 2. Reverse Proxy Core

### ✅ Strengths

* Streaming request body (good)
* Streaming response body
* reqwest connection pooling

### ⚠️ Weak points

* ❌ No upstream connection health tracking
* ❌ No retry policy (fail-fast only)
* ❌ No request hedging
* ❌ No DNS caching control
* ❌ No upstream circuit awareness

👉 Production gap:

> Equivalent to “dumb HTTP forwarder”, not resilient proxy

---

## 3. Load Balancing

### ✅ Strengths

* Round-robin implemented

### ⚠️ Weak points

* ❌ No weighted balancing
* ❌ No health-aware selection
* ❌ No latency-aware routing
* ❌ No retry-aware balancing (stuck nodes still selected)
* ❌ No sticky sessions

👉 Production gap:

> Stateless RR only — not production LB

---

## 4. Middleware Stack

### ✅ Strengths

* Tower-based layering (good foundation)
* Auth middleware exists
* Timeout + rate limit + trace

### ⚠️ Weak points

* ❌ Global middleware only (not per-route)
* ❌ No middleware ordering control per route
* ❌ No context propagation (user claims, trace context)
* ❌ No middleware short-circuit optimization
* ❌ No plugin architecture

👉 Production gap:

> No “policy engine”, only linear filter chain

---

## 5. Resilience

### ⚠️ Missing almost entirely

* ❌ No retry with backoff
* ❌ No circuit breaker
* ❌ No bulkhead isolation
* ❌ No upstream failure scoring
* ❌ No adaptive timeout

👉 Production gap:

> Current system fails immediately — no self-healing behavior

---

## 6. Observability

### ⚠️ Partial

* ✔ TraceLayer logs exist
* ✔ Request ID propagation exists

### Missing

* ❌ No metrics system (Prometheus)
* ❌ No latency histogram
* ❌ No error rate tracking per route
* ❌ No upstream breakdown metrics
* ❌ No distributed tracing (OpenTelemetry spans)

👉 Production gap:

> Logging exists, observability does not

---

## 7. Configuration & Service Discovery

### ⚠️ Weak

* ❌ Static routes only
* ❌ No hot reload
* ❌ No config validation layer
* ❌ No dynamic upstream discovery

👉 Production gap:

> Hardcoded gateway = non-production-ready in cloud environments

---

## 8. Security

### ⚠️ Minimal

* ✔ Header-based auth check

### Missing

* ❌ No JWT validation
* ❌ No RBAC / ABAC
* ❌ No request sanitization
* ❌ No mTLS upstream security
* ❌ No rate-limit identity binding (user-aware throttling)

---

# 🚀 Industry-Grade Gateway TODO (Next Evolution)

This is what your system should become.

---

## 🧠 1. Routing Engine (Upgrade Core)

* [ ] Replace linear route scan with **radix tree router**
* [ ] Precompute route graph at startup
* [ ] Add route metadata:

  * auth required
  * timeout override
  * retry policy
  * rate limit policy
* [ ] Add route caching (O(1) lookup)
* [ ] Support regex routes

---

## 🔁 2. Resilient Proxy Engine

* [ ] Retry policy with exponential backoff + jitter
* [ ] Circuit breaker per upstream
* [ ] Adaptive timeout (based on latency history)
* [ ] Request hedging (parallel upstream calls)
* [ ] DNS caching layer
* [ ] Connection warmup strategy

---

## ⚖️ 3. Smart Load Balancer

* [ ] Weighted round-robin
* [ ] Latency-aware routing (EWMA scoring)
* [ ] Health check integration
* [ ] Sticky session support (cookie/IP hash)
* [ ] Failure-aware routing exclusion

---

## 🧩 4. Middleware System v2 (Policy Engine)

* [ ] Per-route middleware pipeline
* [ ] Middleware composition DSL
* [ ] Context propagation (request-scoped state)
* [ ] Short-circuit optimization (auth/rate-limit first)
* [ ] Plugin-based middleware registry

---

## 🛡 5. Security Layer Upgrade

* [ ] JWT authentication middleware
* [ ] Role-based access control (RBAC)
* [ ] Claim propagation to upstream headers
* [ ] mTLS support for upstream calls
* [ ] Request sanitization layer
* [ ] IP + user-based rate limiting

---

## 📊 6. Observability (Production Requirement)

* [ ] OpenTelemetry integration
* [ ] Distributed tracing (trace → span → upstream span)
* [ ] Prometheus metrics endpoint
* [ ] Histogram:

  * request latency
  * upstream latency
* [ ] Error rate tracking per route/upstream
* [ ] Live dashboards support

---

## ⚙️ 7. Dynamic Configuration System

* [ ] Hot reload route config
* [ ] Config validation layer
* [ ] Versioned config rollout
* [ ] Feature flags per route
* [ ] Runtime config diff reload

---

## 🔥 8. Reliability Layer

* [ ] Bulkhead isolation (per route concurrency limit)
* [ ] Queueing / backpressure handling
* [ ] Load shedding under pressure
* [ ] Failover routing (secondary upstream pools)
* [ ] Graceful degradation mode

---

## 🌐 9. Service Discovery

* [ ] Kubernetes service discovery integration
* [ ] Eureka client integration
* [ ] Consul service registry
* [ ] Health-aware registry sync
* [ ] Automatic upstream deregistration

---

## 📦 10. Performance Optimization

* [ ] HTTP/2 upstream support
* [ ] Connection reuse tuning per host
* [ ] Zero-copy header processing
* [ ] Async batch header filtering
* [ ] Memory pool for request objects

---

# 🧭 Summary (Important)

Your current system is:

> ✔ A **correct reverse proxy with routing + middleware basics**

But not yet:

> ❌ A **production-grade gateway**

The missing core shift is:

### From:

> “request forwarding system”

### To:

> “policy-driven distributed traffic control plane”

---




## Tips

- Start with protocol and contract tests first (request, response, error, timeout).
- Build a strict parser before adding convenience behavior; fail closed on malformed input.
- Add structured request logging early so debugging network paths is cheap.
- Keep connection lifecycle explicit: open, active, idle timeout, close.
- Add load and latency checks after correctness is stable.
