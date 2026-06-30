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

## Completed Projects

The following projects currently have no unchecked items left in their TODO guides.

| Binary | TODO Guide |
|--------|------------|
| `genpass` | [random_password_TODO.md](projects/random_password_TODO.md) |
| `jsonfmt` | [json_formatter_TODO.md](projects/json_formatter_TODO.md) |
| `rgrep` | [mini_grep_TODO.md](projects/mini_grep_TODO.md) |
| `url_shortener` | [url_shortener_TODO.md](projects/url_shortener_TODO.md) |
| `lru_cache` | [lru_cache_TODO.md](projects/lru_cache_TODO.md) |
| `watchdir` | [file_watcher_TODO.md](projects/file_watcher_TODO.md) |
| `logparse` | [log_parser_TODO.md](projects/log_parser_TODO.md) |
| `api_gateway` | [api_gateway_TODO.md](projects/api_gateway_TODO.md) |

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
| 38 | ⭐ 6/10 | `invaders` | [invaders.rs](projects/invaders.rs) | Terminal game loop + rendering |
| 39 | ⭐ 7/10 | `chip8` | [chip8.rs](projects/chip8.rs) | Emulator architecture |
| 40 | ⭐ 8/10 | `browser_engine` | [browser_engine.rs](projects/browser_engine.rs) | Parsing + layout + rendering pipeline |
| 41 | ⭐ 8/10 | `bytecode_vm` | [bytecode_vm.rs](projects/bytecode_vm.rs) | Interpreter internals |
| 42 | ⭐ 8/10 | `mini_compiler` | [mini_compiler.rs](projects/mini_compiler.rs) | Language design + codegen |
| 43 | ⭐ 8/10 | `packet_sniffer` | [packet_sniffer.rs](projects/packet_sniffer.rs) | Packet parsing + traffic analysis |
| 44 | ⭐ 8/10 | `static_site_gen` | [static_site_gen.rs](projects/static_site_gen.rs) | Build pipeline + content generation |
| 45 | ⭐ 9/10 | `iot_simulator` | [iot_simulator.rs](projects/iot_simulator.rs) | Distributed device simulation |
| 46 | ⭐ 10/10 | `raft_consensus` | [raft_consensus.rs](projects/raft_consensus.rs) | Consensus algorithm implementation |
| 47 | ⭐ 5/10 | `markdown_parser` | [markdown_parser.rs](projects/markdown_parser.rs) | Text format parsing |
| 48 | ⭐ 6/10 | `auth_service` | [auth_service.rs](projects/auth_service.rs) | Authentication and session design |
| 49 | ⭐ 7/10 | `ray_tracer` | [ray_tracer.rs](projects/ray_tracer.rs) | Graphics math + rendering |
| 50 | ⭐ 7/10 | `image_codec` | [image_codec.rs](projects/image_codec.rs) | Binary file formats |
| 51 | ⭐ 8/10 | `toy_debugger` | [toy_debugger.rs](projects/toy_debugger.rs) | Process introspection + breakpoints |
| 52 | ⭐ 9/10 | `sqlite_clone` | [sqlite_clone.rs](projects/sqlite_clone.rs) | SQL engine + B-tree storage |
| 53 | ⭐ 9/10 | `wasm_runtime` | [wasm_runtime.rs](projects/wasm_runtime.rs) | WebAssembly decoding + execution |
| 54 | ⭐ 9/10 | `container_runtime` | [container_runtime.rs](projects/container_runtime.rs) | Isolation primitives + resource control |
| 55 | ⭐ 6/10 | `wasm_game_of_life` | [wasm_game_of_life.rs](projects/wasm_game_of_life.rs) | Rust + WebAssembly UI loop |
| 56 | ⭐ 5/10 | `spell_checker` | [spell_checker.rs](projects/spell_checker.rs) | String processing + suggestion ranking |
| 57 | ⭐ 7/10 | `terminal_emulator` | [terminal_emulator.rs](projects/terminal_emulator.rs) | PTY handling + ANSI parsing |
| 58 | ⭐ 8/10 | `nes_emulator` | [nes_emulator.rs](projects/nes_emulator.rs) | Hardware emulation + timing |
| 59 | ⭐ 8/10 | `memory_allocator` | [memory_allocator.rs](projects/memory_allocator.rs) | Allocation strategies + invariants |
| 60 | ⭐ 8/10 | `physics_engine` | [physics_engine.rs](projects/physics_engine.rs) | Simulation + collision resolution |
| 61 | ⭐ 8/10 | `fuse_fs` | [fuse_fs.rs](projects/fuse_fs.rs) | Filesystem semantics + inode modeling |
| 62 | ⭐ 10/10 | `os_kernel` | [os_kernel.rs](projects/os_kernel.rs) | Bare-metal systems programming |
| 63 | ⭐ 7/10 | `ci_system` | [ci_system.rs](projects/ci_system.rs) | Pipeline orchestration + workers |
| 64 | ⭐ 9/10 | `vpn_tunnel` | [vpn_tunnel.rs](projects/vpn_tunnel.rs) | Secure tunneling + peer state |
| 65 | ⭐ 8/10 | `search_engine` | [search_engine.rs](projects/search_engine.rs) | Indexing + relevance ranking |
| 66 | ⭐ 8/10 | `package_manager` | [package_manager.rs](projects/package_manager.rs) | Dependency resolution + artifact lifecycle |
| 67 | ⭐ 9/10 | `graph_database` | [graph_database.rs](projects/graph_database.rs) | Property graph storage + query execution |
| 68 | ⭐ 8/10 | `message_queue` | [message_queue.rs](projects/message_queue.rs) | Durable pub/sub + consumer groups |
| 69 | ⭐ 7/10 | `build_system` | [build_system.rs](projects/build_system.rs) | DAG execution + incremental builds |
| 70 | ⭐ 7/10 | `secret_scanner` | [secret_scanner.rs](projects/secret_scanner.rs) | Credential detection + risk reporting |
| 71 | ⭐ 9/10 | `ebpf_monitor` | [ebpf_monitor.rs](projects/ebpf_monitor.rs) | Kernel telemetry pipeline design |
| 72 | ⭐ 9/10 | `blockchain_node` | [blockchain_node.rs](projects/blockchain_node.rs) | Ledger validation + peer protocol |

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

---

## ⭐ 5/10 — Markdown Parser

Goal: Text format parsing

Build:

```bash
cargo run --bin markdown_parser
```

Learn:

- block and inline parsing
- AST design
- HTML rendering
- snapshot-driven parser testing

Guide:

- [markdown_parser_TODO.md](projects/markdown_parser_TODO.md)

---

## ⭐ 6/10 — Auth Service

Goal: Authentication and session design

Build:

```bash
cargo run --bin auth_service
```

Learn:

- request validation
- password hashing and secret handling
- session or token lifecycle
- middleware-based access control

Guide:

- [auth_service_TODO.md](projects/auth_service_TODO.md)

---

## ⭐ 7/10 — Ray Tracer

Goal: Graphics math + rendering

Build:

```bash
cargo run --bin ray_tracer > image.ppm
```

Learn:

- vector and ray math
- camera and sampling models
- geometry intersections
- material and shading models

Guide:

- [ray_tracer_TODO.md](projects/ray_tracer_TODO.md)

---

## ⭐ 7/10 — Image Codec

Goal: Binary file formats

Build:

```bash
cargo run --bin image_codec
```

Learn:

- chunked binary parsing
- encode/decode symmetry
- checksums and validation
- streaming IO patterns

Guide:

- [image_codec_TODO.md](projects/image_codec_TODO.md)

---

## ⭐ 8/10 — Toy Debugger

Goal: Process introspection + breakpoints

Build:

```bash
cargo run --bin toy_debugger
```

Learn:

- breakpoint management
- register and memory inspection
- stepping and continue flow
- source mapping basics

Guide:

- [toy_debugger_TODO.md](projects/toy_debugger_TODO.md)

---

## ⭐ 9/10 — SQLite Clone

Goal: SQL engine + B-tree storage

Build:

```bash
cargo run --bin sqlite_clone
```

Learn:

- REPL and statement parsing
- row and page layout
- cursor abstraction
- B-tree search and persistence

Guide:

- [sqlite_clone_TODO.md](projects/sqlite_clone_TODO.md)

---

## ⭐ 9/10 — WASM Runtime

Goal: WebAssembly decoding + execution

Build:

```bash
cargo run --bin wasm_runtime
```

Learn:

- binary module decoding
- validation and typing rules
- stack machine execution
- host embedding surface design

Guide:

- [wasm_runtime_TODO.md](projects/wasm_runtime_TODO.md)

---

## ⭐ 9/10 — Container Runtime

Goal: Isolation primitives + resource control

Build:

```bash
cargo run --bin container_runtime
```

Learn:

- namespace setup
- rootfs and mount isolation
- cgroup and rlimit control
- syscall and capability restriction

Guide:

- [container_runtime_TODO.md](projects/container_runtime_TODO.md)

---

## ⭐ 6/10 — WASM Game of Life

Goal: Rust + WebAssembly UI loop

Build:

```bash
cargo run --bin wasm_game_of_life
```

Learn:

- wasm boundary API design
- grid simulation rules
- browser rendering integration
- profiling update and draw cost

Guide:

- [wasm_game_of_life_TODO.md](projects/wasm_game_of_life_TODO.md)

---

## ⭐ 5/10 — Spell Checker

Goal: String processing + suggestion ranking

Build:

```bash
cargo run --bin spell_checker
```

Learn:

- text normalization
- approximate matching
- ranking heuristics
- dictionary-backed CLI design

Guide:

- [spell_checker_TODO.md](projects/spell_checker_TODO.md)

---

## ⭐ 7/10 — Terminal Emulator

Goal: PTY handling + ANSI parsing

Build:

```bash
cargo run --bin terminal_emulator
```

Learn:

- ANSI escape parsing
- screen buffer modeling
- PTY subprocess management
- keyboard and rendering behavior

Guide:

- [terminal_emulator_TODO.md](projects/terminal_emulator_TODO.md)

---

## ⭐ 8/10 — NES Emulator

Goal: Hardware emulation + timing

Build:

```bash
cargo run --bin nes_emulator
```

Learn:

- ROM and bus design
- CPU instruction emulation
- timing-sensitive subsystem coordination
- graphics/input integration

Guide:

- [nes_emulator_TODO.md](projects/nes_emulator_TODO.md)

---

## ⭐ 8/10 — Memory Allocator

Goal: Allocation strategies + invariants

Build:

```bash
cargo run --bin memory_allocator
```

Learn:

- bump and free-list designs
- alignment guarantees
- fragmentation behavior
- allocator trait integration

Guide:

- [memory_allocator_TODO.md](projects/memory_allocator_TODO.md)

---

## ⭐ 8/10 — Physics Engine

Goal: Simulation + collision resolution

Build:

```bash
cargo run --bin physics_engine
```

Learn:

- rigid body modeling
- broad and narrow collision phases
- impulse resolution
- deterministic fixed-step simulation

Guide:

- [physics_engine_TODO.md](projects/physics_engine_TODO.md)

---

## ⭐ 8/10 — FUSE Filesystem

Goal: Filesystem semantics + inode modeling

Build:

```bash
cargo run --bin fuse_fs
```

Learn:

- path and inode resolution
- file and directory operation mapping
- metadata and persistence design
- interface boundary testing

Guide:

- [fuse_fs_TODO.md](projects/fuse_fs_TODO.md)

---

## ⭐ 10/10 — OS Kernel

Goal: Bare-metal systems programming

Build:

```bash
cargo run --bin os_kernel
```

Learn:

- freestanding Rust binaries
- interrupts and low-level output
- paging and memory management
- allocator and executor internals

Guide:

- [os_kernel_TODO.md](projects/os_kernel_TODO.md)

---

## ⭐ 7/10 — CI System

Goal: Pipeline orchestration + workers

Build:

```bash
cargo run --bin ci_system
```

Learn:

- DAG job scheduling
- worker execution and logs
- retry and timeout behavior
- artifacts and workspace propagation

Guide:

- [ci_system_TODO.md](projects/ci_system_TODO.md)

---

## ⭐ 9/10 — VPN Tunnel

Goal: Secure tunneling + peer state

Build:

```bash
cargo run --bin vpn_tunnel
```

Learn:

- encapsulation and packet flow
- handshake and session lifecycle
- replay protection and rekeying
- virtual interface and routing model

Guide:

- [vpn_tunnel_TODO.md](projects/vpn_tunnel_TODO.md)

---

## ⭐ 8/10 — Search Engine

Goal: Indexing + relevance ranking

Build:

```bash
cargo run --bin search_engine
```

Learn:

- tokenization and normalization pipeline design
- inverted index and posting list structures
- BM25-style scoring and ranking behavior
- segment merge and incremental indexing workflow

Guide:

- [search_engine_TODO.md](projects/search_engine_TODO.md)

---

## ⭐ 8/10 — Package Manager

Goal: Dependency resolution + artifact lifecycle

Build:

```bash
cargo run --bin package_manager
```

Learn:

- semantic version constraint solving
- lockfile determinism and reproducibility
- package cache layout and integrity checks
- install/update/remove lifecycle operations

Guide:

- [package_manager_TODO.md](projects/package_manager_TODO.md)

---

## ⭐ 9/10 — Graph Database

Goal: Property graph storage + query execution

Build:

```bash
cargo run --bin graph_database
```

Learn:

- node/edge storage and property indexing
- graph pattern query planning basics
- traversal execution with filtering and projection
- snapshot and recovery design

Guide:

- [graph_database_TODO.md](projects/graph_database_TODO.md)

---

## ⭐ 6/10 — Invaders

Goal: Terminal game loop + rendering

Build:

```bash
cargo run --bin invaders
```

Learn:

- realtime input loop and frame timing
- terminal rendering with double-buffer style updates
- collision checks between player shots and enemies
- audio cues and alternate-screen cleanup

Guide:

- [invaders_TODO.md](projects/invaders_TODO.md)

---

## ⭐ 8/10 — Message Queue

Goal: Durable pub/sub + consumer groups

Build:

```bash
cargo run --bin message_queue
```

Learn:

- topic partitioning and append-only logs
- producer ordering and consumer offset tracking
- redelivery semantics and failure recovery
- retention policy and segment cleanup

Guide:

- [message_queue_TODO.md](projects/message_queue_TODO.md)

---

## ⭐ 7/10 — Build System

Goal: DAG execution + incremental builds

Build:

```bash
cargo run --bin build_system
```

Learn:

- target graph construction and cycle detection
- topological scheduling with parallel workers
- content-hash-based cache invalidation
- artifact cache reuse and reproducibility

Guide:

- [build_system_TODO.md](projects/build_system_TODO.md)

---

## ⭐ 7/10 — Secret Scanner

Goal: Credential detection + risk reporting

Build:

```bash
cargo run --bin secret_scanner
```

Learn:

- repository traversal with ignore semantics
- signature-based and entropy-based detectors
- finding triage with severity and confidence
- baseline and suppressions workflow

Guide:

- [secret_scanner_TODO.md](projects/secret_scanner_TODO.md)

---

## ⭐ 9/10 — eBPF Monitor

Goal: Kernel telemetry pipeline design

Build:

```bash
cargo run --bin ebpf_monitor
```

Learn:

- event schema design across process and network signals
- ring buffer style event transport to userspace
- attach/detach lifecycle and safe fallback behavior
- overload handling with bounded queues and drop metrics

Guide:

- [ebpf_monitor_TODO.md](projects/ebpf_monitor_TODO.md)

---

## ⭐ 9/10 — Blockchain Node

Goal: Ledger validation + peer protocol

Build:

```bash
cargo run --bin blockchain_node
```

Learn:

- transaction and block validation rules
- mempool management and block assembly
- peer gossip and fork-choice handling
- durable chain state and crash recovery

Guide:

- [blockchain_node_TODO.md](projects/blockchain_node_TODO.md)
