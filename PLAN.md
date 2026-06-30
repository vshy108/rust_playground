# Rust Playground — Project Learning Path

Projects are roughly ordered by difficulty. Each builds on concepts from the ones before it.

---

## Fixtures

Shared test data in `fixtures/` — run commands from the repo root so relative paths resolve correctly.

| File | Used by | Description |
|------|---------|-------------|
| [fixtures/sample.txt](fixtures/sample.txt) | `rgrep` | 9 lines of plain text; mix of `hello`, `error`, `warning`, `debug` prefixes for plain and regex match testing |
| [fixtures/compact.json](fixtures/compact.json) | `jsonfmt` | Valid compact JSON (happy path for pretty-print and `--check`) |
| [fixtures/bad.json](fixtures/bad.json) | `jsonfmt` | JSON with a missing closing brace (error path) |
| [fixtures/access.log](fixtures/access.log) | `logparse` | 20 lines of Common Log Format; mix of IPs and status codes, one intentionally malformed line for skip-and-warn testing |

## Tips For Incomplete TODO Projects

TODO guide convention reference: [TODO_RULES.md](TODO_RULES.md)

Use this workflow whenever a project has unchecked milestones:

1. Start from one milestone only. Pick the smallest observable behavior and ignore the rest.
2. Add a failing test first for that behavior (`cargo test --bin <name>`), then implement the smallest fix.
3. Keep boundaries explicit early (parse -> validate -> execute -> output) so refactors stay local.
4. Prefer deterministic inputs and fixtures before adding randomness, concurrency, or networking.
5. Add debug surfaces early (structured logs, `--verbose`, or trace output) to shorten feedback loops.
6. Validate after every slice: `cargo check --bins` and focused bin tests before moving forward.
7. When blocked, reduce scope instead of pausing: implement a reduced but complete version, then iterate.

Definition of done for each milestone:

- behavior is tested
- error path is tested
- CLI or API contract is documented in the project TODO

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
| 20 | ⭐ 5/10 | `dns_toolkit` | [dns_toolkit.rs](projects/dns_toolkit.rs) | Binary protocol + UDP networking |
| 21 | ⭐ 5/10 | `mini_git` | [mini_git.rs](projects/mini_git.rs) | Content-addressed storage |
| 22 | ⭐ 5/10 | `loadtest` | [http_load_tester.rs](projects/http_load_tester.rs) | Performance testing + latency stats |
| 23 | ⭐ 6/10 | `job_queue` | [job_queue.rs](projects/job_queue.rs) | Retries, DLQ, worker orchestration |
| 24 | ⭐ 6/10 | `inv_index` | [inverted_index.rs](projects/inverted_index.rs) | Search indexing + ranking |
| 25 | ⭐ 6/10 | `mini_shell` | [mini_shell.rs](projects/mini_shell.rs) | Process control + pipes |
| 26 | ⭐ 7/10 | `textedit` | [text_editor.rs](projects/text_editor.rs) | Terminal UI + buffer editing |
| 27 | ⭐ 7/10 | `socks5_proxy` | [socks5_proxy.rs](projects/socks5_proxy.rs) | Protocol parsing + TCP proxying |
| 28 | ⭐ 7/10 | `ws_broker` | [websocket_broker.rs](projects/websocket_broker.rs) | Realtime pub/sub patterns |
| 29 | ⭐ 7/10 | `rate_limiter` | [rate_limiter.rs](projects/rate_limiter.rs) | Traffic shaping algorithms |
| 30 | ⭐ 8/10 | `lsm_kv` | [lsm_kv.rs](projects/lsm_kv.rs) | Storage engine internals |
| 31 | ⭐ 8/10 | `bloom_hll` | [bloom_hll.rs](projects/bloom_hll.rs) | Probabilistic data structures |
| 32 | ⭐ 8/10 | `file_sync` | [file_sync.rs](projects/file_sync.rs) | Filesystem diff + sync engine |
| 33 | ⭐ 8/10 | `template_engine` | [template_engine.rs](projects/template_engine.rs) | Parsing + AST + rendering |
| 34 | ⭐ 9/10 | `regex_engine` | [regex_engine.rs](projects/regex_engine.rs) | Automata theory in practice |
| 35 | ⭐ 9/10 | `bittorrent` | [bittorrent.rs](projects/bittorrent.rs) | P2P protocols + piece scheduling |
| 36 | ⭐ 7/10 | `tcp_stack` | [tcp_stack.rs](projects/tcp_stack.rs) | Network stack internals |
| 37 | ⭐ 7/10 | `mqtt_broker` | [mqtt_broker.rs](projects/mqtt_broker.rs) | Pub/sub protocol server design |
| 38 | ⭐ 7/10 | `chip8` | [chip8.rs](projects/chip8.rs) | Emulator architecture |
| 39 | ⭐ 8/10 | `browser_engine` | [browser_engine.rs](projects/browser_engine.rs) | Parsing + layout + rendering pipeline |
| 40 | ⭐ 8/10 | `bytecode_vm` | [bytecode_vm.rs](projects/bytecode_vm.rs) | Interpreter internals |
| 41 | ⭐ 8/10 | `mini_compiler` | [mini_compiler.rs](projects/mini_compiler.rs) | Language design + codegen |
| 42 | ⭐ 8/10 | `packet_sniffer` | [packet_sniffer.rs](projects/packet_sniffer.rs) | Packet parsing + traffic analysis |
| 43 | ⭐ 8/10 | `static_site_gen` | [static_site_gen.rs](projects/static_site_gen.rs) | Build pipeline + content generation |
| 44 | ⭐ 9/10 | `iot_simulator` | [iot_simulator.rs](projects/iot_simulator.rs) | Distributed device simulation |
| 45 | ⭐ 10/10 | `raft_consensus` | [raft_consensus.rs](projects/raft_consensus.rs) | Consensus algorithm implementation |

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

---

## ⭐ 5/10 — DNS Toolkit

Goal: Binary protocol + UDP networking

Build:

```bash
cargo run --bin dns_toolkit
```

Learn:

- DNS packet parsing / encoding
- UDP request/response flow
- timeout + retry behavior
- TTL cache behavior

Guide:

- [dns_toolkit_TODO.md](projects/dns_toolkit_TODO.md)

---

## ⭐ 5/10 — Mini Git

Goal: Content-addressed storage

Build:

```bash
cargo run --bin mini_git -- init
```

Learn:

- hashing and object storage
- staging/index design
- commit graph fundamentals
- repository state transitions

Guide:

- [mini_git_TODO.md](projects/mini_git_TODO.md)

---

## ⭐ 5/10 — HTTP Load Tester

Goal: Performance testing + latency stats

Build:

```bash
cargo run --bin loadtest -- --url http://127.0.0.1:3000 --concurrency 32 --requests 2000
```

Learn:

- async worker orchestration
- latency histogram calculations
- throughput and error-rate reporting
- client connection reuse

Guide:

- [http_load_tester_TODO.md](projects/http_load_tester_TODO.md)

---

## ⭐ 6/10 — Job Queue

Goal: Retries, DLQ, worker orchestration

Build:

```bash
cargo run --bin job_queue
```

Learn:

- lease/ack semantics
- retry policy design
- dead-letter handling
- graceful shutdown behavior

Guide:

- [job_queue_TODO.md](projects/job_queue_TODO.md)

---

## ⭐ 6/10 — Inverted Index

Goal: Search indexing + ranking

Build:

```bash
cargo run --bin inv_index -- fixtures/sample.txt
```

Learn:

- tokenization and normalization
- postings list construction
- query parsing
- BM25-style scoring

Guide:

- [inverted_index_TODO.md](projects/inverted_index_TODO.md)

---

## ⭐ 6/10 — Mini Shell

Goal: Process control + pipes

Build:

```bash
cargo run --bin mini_shell
```

Learn:

- REPL architecture
- command parsing and quoting
- subprocess management
- pipelines and redirection

Guide:

- [mini_shell_TODO.md](projects/mini_shell_TODO.md)

---

## ⭐ 7/10 — Text Editor

Goal: Terminal UI + buffer editing

Build:

```bash
cargo run --bin textedit -- README.md
```

Learn:

- viewport rendering
- cursor + selection safety
- insertion/deletion primitives
- save/search workflows

Guide:

- [text_editor_TODO.md](projects/text_editor_TODO.md)

---

## ⭐ 7/10 — SOCKS5 Proxy

Goal: Protocol parsing + TCP proxying

Build:

```bash
cargo run --bin socks5_proxy -- --listen 127.0.0.1:1080
```

Learn:

- SOCKS5 handshake and connect flow
- target address parsing
- bidirectional stream copying
- timeout and connection accounting

Guide:

- [socks5_proxy_TODO.md](projects/socks5_proxy_TODO.md)

---

## ⭐ 7/10 — WebSocket Broker

Goal: Realtime pub/sub patterns

Build:

```bash
cargo run --bin ws_broker
```

Learn:

- websocket session lifecycle
- pub/sub topic routing
- backpressure handling
- heartbeat/cleanup loops

Guide:

- [websocket_broker_TODO.md](projects/websocket_broker_TODO.md)

---

## ⭐ 7/10 — Rate Limiter

Goal: Traffic shaping algorithms

Build:

```bash
cargo run --bin rate_limiter
```

Learn:

- token bucket implementation
- sliding-window implementation
- per-key state management
- middleware integration patterns

Guide:

- [rate_limiter_TODO.md](projects/rate_limiter_TODO.md)

---

## ⭐ 8/10 — LSM KV Store

Goal: Storage engine internals

Build:

```bash
cargo run --bin lsm_kv
```

Learn:

- memtable + WAL layering
- SSTable read/write format
- compaction + tombstones
- crash recovery

Guide:

- [lsm_kv_TODO.md](projects/lsm_kv_TODO.md)

---

## ⭐ 8/10 — Bloom + HyperLogLog

Goal: Probabilistic data structures

Build:

```bash
cargo run --bin bloom_hll
```

Learn:

- approximate membership checks
- cardinality estimation
- merge behavior correctness
- precision/performance tradeoffs

Guide:

- [bloom_hll_TODO.md](projects/bloom_hll_TODO.md)

---

## ⭐ 8/10 — File Sync

Goal: Filesystem diff + sync engine

Build:

```bash
cargo run --bin file_sync -- --src ./fixtures --dst /tmp/sync_target
```

Learn:

- directory manifest modeling
- hashing and delta detection
- atomic file replacement
- bounded parallel copy

Guide:

- [file_sync_TODO.md](projects/file_sync_TODO.md)

---

## ⭐ 8/10 — Template Engine

Goal: Parsing + AST + rendering

Build:

```bash
cargo run --bin template_engine
```

Learn:

- tokenization and parser design
- AST traversal
- context resolution
- conditional/loop control blocks

Guide:

- [template_engine_TODO.md](projects/template_engine_TODO.md)

---

## ⭐ 9/10 — Regex Engine

Goal: Automata theory in practice

Build:

```bash
cargo run --bin regex_engine -- "a+b" "aaab"
```

Learn:

- regex grammar parsing
- NFA construction
- NFA simulation matching
- optional DFA conversion path

Guide:

- [regex_engine_TODO.md](projects/regex_engine_TODO.md)

---

## ⭐ 9/10 — BitTorrent Client Core

Goal: P2P protocols + piece scheduling

Build:

```bash
cargo run --bin bittorrent
```

Learn:

- bencode parsing
- tracker/peer coordination
- piece verification and assembly
- rarest-first scheduling strategy

Guide:

- [bittorrent_TODO.md](projects/bittorrent_TODO.md)

---

## ⭐ 7/10 — TCP/IP Stack (Toy)

Goal: Network stack internals

Build:

```bash
cargo run --bin tcp_stack
```

Learn:

- packet parsing and encoding
- layered protocol boundaries
- TCP handshake state transitions
- retransmission and timeout modeling

Guide:

- [tcp_stack_TODO.md](projects/tcp_stack_TODO.md)

---

## ⭐ 7/10 — MQTT Broker

Goal: Pub/sub protocol server design

Build:

```bash
cargo run --bin mqtt_broker
```

Learn:

- binary protocol framing
- topic routing with wildcards
- session lifecycle and keepalive
- QoS delivery guarantees

Guide:

- [mqtt_broker_TODO.md](projects/mqtt_broker_TODO.md)

---

## ⭐ 7/10 — CHIP-8 Emulator

Goal: Emulator architecture

Build:

```bash
cargo run --bin chip8
```

Learn:

- instruction decode and execute loop
- memory/register modeling
- deterministic timer behavior
- ROM loading and compatibility checks

Guide:

- [chip8_TODO.md](projects/chip8_TODO.md)

---

## ⭐ 8/10 — Browser Engine (Toy)

Goal: Parsing + layout + rendering pipeline

Build:

```bash
cargo run --bin browser_engine
```

Learn:

- HTML/CSS parsing
- DOM and style tree construction
- box-model layout traversal
- paint command generation

Guide:

- [browser_engine_TODO.md](projects/browser_engine_TODO.md)

---

## ⭐ 8/10 — Bytecode VM

Goal: Interpreter internals

Build:

```bash
cargo run --bin bytecode_vm
```

Learn:

- VM instruction dispatch
- frame and stack discipline
- call/return mechanics
- debugging with disassembly

Guide:

- [bytecode_vm_TODO.md](projects/bytecode_vm_TODO.md)

---

## ⭐ 8/10 — Mini Compiler

Goal: Language design + codegen

Build:

```bash
cargo run --bin mini_compiler
```

Learn:

- lexing and parsing strategy
- AST + semantic checks
- IR or bytecode generation
- execution and correctness validation

Guide:

- [mini_compiler_TODO.md](projects/mini_compiler_TODO.md)

---

## ⭐ 8/10 — Packet Sniffer

Goal: Packet parsing + traffic analysis

Build:

```bash
cargo run --bin packet_sniffer
```

Learn:

- packet capture pipeline
- robust header parsing
- query/filter model design
- flow-level aggregation metrics

Guide:

- [packet_sniffer_TODO.md](projects/packet_sniffer_TODO.md)

---

## ⭐ 8/10 — Static Site Generator

Goal: Build pipeline + content generation

Build:

```bash
cargo run --bin static_site_gen
```

Learn:

- markdown + front matter parsing
- template composition
- deterministic output generation
- incremental rebuild logic

Guide:

- [static_site_gen_TODO.md](projects/static_site_gen_TODO.md)

---

## ⭐ 9/10 — IoT Device Simulator

Goal: Distributed device simulation

Build:

```bash
cargo run --bin iot_simulator
```

Learn:

- deterministic event scheduling
- virtual sensor/actuator abstractions
- network fault injection
- scenario-driven simulation testing

Guide:

- [iot_simulator_TODO.md](projects/iot_simulator_TODO.md)

---

## ⭐ 10/10 — Raft Consensus

Goal: Consensus algorithm implementation

Build:

```bash
cargo run --bin raft_consensus
```

Learn:

- leader election and term management
- replicated log consistency
- persistence and crash recovery model
- safety/liveness test design

Guide:

- [raft_consensus_TODO.md](projects/raft_consensus_TODO.md)
