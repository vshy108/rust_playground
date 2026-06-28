# Rust Playground — Project Learning Path

Projects are ordered by difficulty. Each builds on concepts from the ones before it.

---

## Fixtures

Shared test data in `fixtures/` — run commands from the repo root so relative paths resolve correctly.

| File | Used by | Description |
|------|---------|-------------|
| [fixtures/sample.txt](fixtures/sample.txt) | `rgrep` | 9 lines of plain text; mix of `hello`, `error`, `warning`, `debug` prefixes for plain and regex match testing |
| [fixtures/compact.json](fixtures/compact.json) | `jsonfmt` | Valid compact JSON (happy path for pretty-print and `--check`) |
| [fixtures/bad.json](fixtures/bad.json) | `jsonfmt` | JSON with a missing closing brace (error path) |
| [fixtures/access.log](fixtures/access.log) | `logparse` | 20 lines of Common Log Format; mix of IPs and status codes, one intentionally malformed line for skip-and-warn testing |

| # | Rating | Binary | Source | Goal |
|---|--------|--------|--------|------|
| 1 | ⭐ 1/10 | `genpass` | [random_password_cli.rs](projects/random_password_cli.rs) | Rust basics |
| 2 | ⭐ 2/10 | `jsonfmt` | [json_formatter_cli.rs](projects/json_formatter_cli.rs) | Serialization with serde |
| 3 | ⭐ 2/10 | `rgrep` | [mini_grep_cli.rs](projects/mini_grep_cli.rs) | Ownership + iterators |
| 4 | ⭐ 3/10 | `url_shortener` | [url_shortener.rs](projects/url_shortener.rs) | Data modeling |
| 5 | ⭐ 3/10 | `lru_cache` | [lru_cache.rs](projects/lru_cache.rs) | Ownership thinking |
| 6 | ⭐ 4/10 | `watchdir` | [file_watcher_cli.rs](projects/file_watcher_cli.rs) | Filesystem events + channels + debounce |
| 7 | ⭐ 4/10 | `logparse` | [log_parser_cli.rs](projects/log_parser_cli.rs) | Iterators + parsing + aggregation |
| 8 | ⭐ 5/10 | `rest_api` | [rest_api.rs](projects/rest_api.rs) | Production backend |
| 9 | ⭐ 5/10 | `crawler` | [web_crawler.rs](projects/web_crawler.rs) | Async mindset |
| 10 | ⭐ 6/10 | `redis_clone` | [redis_clone.rs](projects/redis_clone.rs) | Network + memory |
| 11 | ⭐ 6/10 | `metrics_collector` | [metrics_collector.rs](projects/metrics_collector.rs) | Observability |
| 12 | ⭐ 7/10 | `api_gateway` | [api_gateway.rs](projects/api_gateway.rs) | Architecture |
| 13 | ⭐ 7/10 | `kafka_consumer` | [kafka_consumer.rs](projects/kafka_consumer.rs) | Enterprise patterns |
| 14 | ⭐ 8/10 | `workflow_engine` | [workflow_engine.rs](projects/workflow_engine.rs) | State machines + DAG |
| 15 | ⭐ 8/10 | `otel_collector` | [otel_collector.rs](projects/otel_collector.rs) | Infra + streaming |
| 16 | ⭐ 9/10 | `fake_sql_server` | [fake_sql_server.rs](projects/fake_sql_server.rs) | Protocol engineering |
| 17 | ⭐ 9/10 | `sidecar` | [service_mesh_sidecar.rs](projects/service_mesh_sidecar.rs) | Networking mastery |
| 18 | ⭐ 10/10 | `dist_cache` | [dist_cache.rs](projects/dist_cache.rs) | Rust architect level |
| 19 | ⭐ 10/10 | `mini_runtime` | [mini_runtime.rs](projects/mini_runtime.rs) | Deep Rust internals |

---

## ⭐ 1/10 — Random Password CLI

Goal: Rust basics

Build:

```bash
cargo run --bin genpass -- --length 20
```

Learn:

- `String` vs `&str`
- `Vec` + iterators
- `rand`
- argument parsing with `std::env::args()`
- `Result<T, E>` + error propagation

Extra:

- symbols toggle (`--symbols`)
- `--help` flag

---

## ⭐ 2/10 — JSON Formatter

Goal: Serialization with serde

Build:

```bash
cargo run --bin jsonfmt -- fixtures/compact.json
```

Learn:

- `serde` / `serde_json`
- `Result` + `?` operator
- file IO with `fs::read_to_string`
- error propagation with `Box<dyn Error>`

Extra:

- pretty-print
- validate-only mode (`--check`)

---

## ⭐ 2/10 — Mini Grep

Goal: Ownership + iterators

Build:

```bash
cargo run --bin rgrep -- hello fixtures/sample.txt
```

Learn:

- borrowing
- iterators + `filter` + `collect`
- slices (`&[String]`)
- lifetime elision

Extra:

- regex matching (`regex` crate)

---

## ⭐ 3/10 — URL Shortener (in-memory)

Goal: Data modeling

API:

```
POST /shorten
GET /:code
```

Learn:

- `HashMap`
- structs + `serde`
- `uuid`
- `Arc<Mutex<T>>` shared state
- `axum` routing

Extra:

- expiration / TTL

---

## ⭐ 3/10 — LRU Cache

Goal: Ownership thinking

Build:

```bash
cargo run --bin lru_cache
```

Learn:

- `HashMap` + `Vec` arena pattern
- `Option<T>`
- mutability + borrow checker
- `std::time::Instant` + `Duration`

Extra:

- TTL-based eviction

---

## ⭐ 4/10 — File Watcher

Goal: Filesystem events + channels + debounce

Build:

```bash
cargo run --bin watchdir -- fixtures/
```

Learn:

- `notify` crate — cross-platform filesystem watcher
- `std::sync::mpsc` channels
- `recv_timeout` + debounce with `HashMap`
- `ctrlc` — graceful shutdown signal
- `FnMut` closure trait
- loop as expression (`break value`)

Extra:

- configurable debounce window

---

## ⭐ 4/10 — Log Parser CLI

Goal: Iterators + parsing + aggregation

Build:

```bash
cargo run --bin logparse -- fixtures/access.log
```

Learn:

- line-by-line file parsing
- `HashMap` counters (top IPs)
- mean + p99 latency
- error rate calculation
- iterator chaining

Extra:

- CSV export (`--csv`)

---

## ⭐ 5/10 — REST API

Goal: Production backend

API:

```
POST   /items
GET    /items
GET    /items/:id
PUT    /items/:id
DELETE /items/:id
```

Learn:

- `serde` data models
- `Arc<Mutex<HashMap>>` shared store
- axum CRUD handler patterns
- integration tests with `tower::ServiceExt::oneshot`

Extra:

- JWT authentication middleware

---

## ⭐ 5/10 — Web Crawler

Goal: Async mindset

Build:

```bash
cargo run --bin crawler -- https://example.com
```

Learn:

- async HTTP with `reqwest`
- HTML link extraction
- BFS graph traversal
- `tokio::task::spawn` concurrency
- URL resolution

Extra:

- concurrency limiting with `tokio::sync::Semaphore`

---

## ⭐ 6/10 — Redis Clone

Goal: Network + memory

Build:

```bash
cargo run --bin redis_clone
redis-cli SET foo bar
redis-cli GET foo
```

Learn:

- RESP binary protocol parsing
- TCP server with `tokio::net::TcpListener`
- `Arc<Mutex<HashMap>>` shared state
- command dispatch table

Extra:

- persistence: serialize to JSON on SIGTERM, restore on startup

---

## ⭐ 6/10 — Metrics Collector

Goal: Observability

Build:

```bash
cargo run --bin metrics_collector
```

Learn:

- metric types enum (counter, gauge, histogram)
- aggregator task with `mpsc` channel
- lock-free snapshot queries
- min / max / sum / mean computation

Extra:

- Prometheus text exposition at `GET /metrics`

---

## ⭐ 7/10 — API Gateway

Goal: Architecture

Build:

```bash
cargo run --bin api_gateway
```

Learn:

- longest-prefix route matching
- reverse proxy with `reqwest`
- timeout wrapping
- exponential backoff retry
- sliding-window rate limiting

Extra:

- OpenTelemetry W3C Trace Context propagation

---

## ⭐ 7/10 — Kafka Consumer

Goal: Enterprise patterns

Build:

```bash
cargo run --bin kafka_consumer
```

Learn:

- message model + simulated broker (mpsc channel)
- async worker pool
- retry exhaustion logic
- dead-letter queue (DLQ)

Extra:

- structured tracing with `tracing` crate

---

## ⭐ 8/10 — Workflow Engine

Goal: State machines + DAG

Build:

```bash
cargo run --bin workflow_engine
```

Learn:

- graph model: `NodeId` / `Step` / `Workflow`
- Kahn's topological sort
- cycle detection
- `NodeState` enum
- async batch execution

Extra:

- live terminal UI with ANSI / `crossterm`

---

## ⭐ 8/10 — OTel Collector

Goal: Infra + streaming

Build:

```bash
cargo run --bin otel_collector
```

Learn:

- span model with `serde`
- HTTP POST receiver
- batch buffer: size + time flush
- NDJSON export

Extra:

- metrics endpoint + periodic summary (spans/min)

---

## ⭐ 9/10 — Fake SQL Server

Goal: Protocol engineering

Build:

```bash
cargo run --bin fake_sql_server
psql -h localhost -p 5433
```

Learn:

- binary message codec (tag + length-prefixed body)
- PostgreSQL startup handshake
- `Query` message loop
- `RowDescription` + `DataRow` framing

Extra:

- SQL parsing: `SELECT <expr> FROM <table>`

---

## ⭐ 9/10 — Service Mesh Sidecar

Goal: Networking mastery

Build:

```bash
cargo run --bin sidecar
```

Learn:

- TCP transparent proxy (`tokio::io::copy`)
- byte / latency / error counters per connection
- global totals with `Arc<AtomicU64>`
- metrics HTTP endpoint

Extra:

- host-based routing via HTTP `Host` header

---

## ⭐ 10/10 — Distributed Cache

Goal: Rust architect level

Build:

```bash
cargo run --bin dist_cache
```

Learn:

- single-node in-memory cache
- consistent hashing ring
- peer join + discovery
- replication (configurable N writes)
- failover detection

Extra:

- quorum-based consistency: refuse writes when < W nodes reachable

---

## ⭐ 10/10 — Mini Async Runtime

Goal: Deep Rust internals

Build:

```bash
cargo run --bin mini_runtime
```

Learn:

- manual `TimerFuture` with `Pending` / `Ready`
- `Task` wrapper + `Wake` trait
- single-threaded executor loop
- task interleaving

Extra:

- multi-threaded scheduler with work-stealing (`crossbeam-deque`)
