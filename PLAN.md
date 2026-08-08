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

## Project Rating Rubric (1-10)

Use this rubric for consistent ratings across projects.

Scoring model:

- 5 categories, each scored 0-2 points.
- Total score = sum of all categories (0-10).

### 1) Implementation depth

- 0: Scaffold only (placeholder main, no real logic).
- 1: Partial implementation, but no complete end-to-end flow.
- 2: Core end-to-end flow works.

### 2) Core behavior coverage

- 0: Most required behaviors are missing.
- 1: Some required behaviors work.
- 2: Multiple core behaviors work together coherently.

### 3) Reliability and safety

- 0: Fragile (panic-prone, no limits/guards).
- 1: Basic defensive handling exists.
- 2: Clear bounds, defensive checks, and stable runtime behavior.

### 4) Maintainability and code clarity

- 0: Mostly ad-hoc structure and magic numbers.
- 1: Some structure and readability improvements.
- 2: Clean structure (constants/helpers), readable flow, low drift risk.

### 5) Validation and project clarity

- 0: No meaningful TODO/test/acceptance guidance.
- 1: Partial TODO clarity or manual-only validation.
- 2: Clear TODO milestones and acceptance checks, with focused verification.

Practical anchors:

- `1/10` usually means scaffold-only with no meaningful implementation.
- `8/10` usually means real working core behavior with good structure, but missing advanced depth (for example broad tests, persistence, or edge-case hardening).

## Completed Projects

The following projects currently have no unchecked items left in their TODO guides.

| Binary | TODO Guide |
|--------|------------|
| `genpass` | [random_password_TODO.md](projects/completed/random_password_TODO.md) |
| `jsonfmt` | [json_formatter_TODO.md](projects/completed/json_formatter_TODO.md) |
| `rgrep` | [mini_grep_TODO.md](projects/completed/mini_grep_TODO.md) |
| `url_shortener` | [url_shortener_TODO.md](projects/completed/url_shortener_TODO.md) |
| `lru_cache` | [lru_cache_TODO.md](projects/completed/lru_cache_TODO.md) |
| `watchdir` | [file_watcher_TODO.md](projects/completed/file_watcher_TODO.md) |
| `logparse` | [log_parser_TODO.md](projects/completed/log_parser_TODO.md) |
| `api_gateway` | [api_gateway_TODO.md](projects/completed/api_gateway_TODO.md) |
| `calculator_cli` | [calculator_cli_TODO.md](projects/completed/calculator_cli_TODO.md) |

Note: Projects below are sorted by rating in ascending order to match the learning workflow.

| # | Rating | Status | Binary | Source | Goal |
|---|--------|--------|--------|--------|------|
| 1 | ⭐ 1/10 | Completed | `genpass` | [random_password_cli.rs](projects/completed/random_password_cli.rs) | Rust basics |
| 2 | ⭐ 2/10 | Completed | `jsonfmt` | [json_formatter_cli.rs](projects/completed/json_formatter_cli.rs) | Serialization with serde |
| 3 | ⭐ 2/10 | Completed | `rgrep` | [mini_grep_cli.rs](projects/completed/mini_grep_cli.rs) | Ownership + iterators |
| 4 | ⭐ 2/10 | Completed | `calculator_cli` | [calculator_cli.rs](projects/completed/calculator_cli.rs) | Evaluate basic arithmetic expressions |
| 5 | ⭐ 2/10 | Planned | `timer_cli` | [timer_cli.rs](projects/planned/rating_2/timer_cli.rs) | Countdown and stopwatch utility |
| 6 | ⭐ 2/10 | Planned | `base64_cli` | [base64_cli.rs](projects/planned/rating_2/base64_cli.rs) | Encode and decode Base64 data |
| 7 | ⭐ 2/10 | Planned | `uuid_generator` | [uuid_generator.rs](projects/planned/rating_2/uuid_generator.rs) | Generate UUIDs from CLI |
| 8 | ⭐ 2/10 | Planned | `gitignore_gen` | [gitignore_gen.rs](projects/planned/rating_2/gitignore_gen.rs) | Generate `.gitignore` presets |
| 9 | ⭐ 2/10 | Planned | `color_preview` | [color_preview.rs](projects/planned/rating_2/color_preview.rs) | Preview colors in the terminal |
| 10 | ⭐ 2/10 | Planned | `wc_clone` | [wc_clone.rs](projects/planned/rating_2/wc_clone.rs) | Count lines, words, and bytes |
| 11 | ⭐ 2/10 | Planned | `slug_generator` | [slug_generator.rs](projects/planned/rating_2/slug_generator.rs) | Generate URL-friendly slugs |
| 12 | ⭐ 2/10 | Planned | `ascii_table` | [ascii_table.rs](projects/planned/rating_2/ascii_table.rs) | Print an ASCII reference table |
| 13 | ⭐ 2/10 | Planned | `url_codec` | [url_codec.rs](projects/planned/rating_2/url_codec.rs) | Encode and decode URL components |
| 14 | ⭐ 3/10 | Completed | `url_shortener` | [url_shortener.rs](projects/completed/url_shortener.rs) | Data modeling |
| 15 | ⭐ 3/10 | Completed | `lru_cache` | [lru_cache.rs](projects/completed/lru_cache.rs) | Ownership thinking |
| 16 | ⭐ 3/10 | Planned | `unit_converter` | [unit_converter.rs](projects/planned/rating_3/unit_converter.rs) | Simple unit conversions from CLI input |
| 17 | ⭐ 3/10 | Planned | `checksum_tool` | [checksum_tool.rs](projects/planned/rating_3/checksum_tool.rs) | File hashing + checksum verification |
| 18 | ⭐ 3/10 | Planned | `weather_cli` | [weather_cli.rs](projects/planned/rating_3/weather_cli.rs) | Current weather + short forecast output |
| 19 | ⭐ 3/10 | Planned | `duplicate_line_filter` | [duplicate_line_filter.rs](projects/planned/rating_3/duplicate_line_filter.rs) | Remove or count repeated lines |
| 20 | ⭐ 3/10 | Planned | `json_viewer` | [json_viewer.rs](projects/planned/rating_3/json_viewer.rs) | Pretty-print JSON data |
| 21 | ⭐ 3/10 | Planned | `csv_pretty_printer` | [csv_pretty_printer.rs](projects/planned/rating_3/csv_pretty_printer.rs) | Render CSV as aligned tables |
| 22 | ⭐ 3/10 | Planned | `file_splitter` | [file_splitter.rs](projects/planned/rating_3/file_splitter.rs) | Split files by size or lines |
| 23 | ⭐ 3/10 | Planned | `env_linter` | [env_linter.rs](projects/planned/rating_3/env_linter.rs) | Detect common `.env` file issues |
| 24 | ⭐ 3/10 | Planned | `hex_viewer` | [hex_viewer.rs](projects/planned/rating_3/hex_viewer.rs) | Inspect binary files in hex |
| 25 | ⭐ 3/10 | Planned | `file_finder` | [file_finder.rs](projects/planned/rating_3/file_finder.rs) | Recursively search files by pattern |
| 26 | ⭐ 3/10 | Planned | `line_ending_converter` | [line_ending_converter.rs](projects/planned/rating_3/line_ending_converter.rs) | Convert text line endings |
| 27 | ⭐ 3/10 | Planned | `notes_cli` | [notes_cli.rs](projects/planned/rating_3/notes_cli.rs) | Store and manage simple notes |
| 28 | ⭐ 4/10 | Completed | `watchdir` | [file_watcher_cli.rs](projects/completed/file_watcher_cli.rs) | Filesystem events + channels + debounce |
| 29 | ⭐ 4/10 | Completed | `logparse` | [log_parser_cli.rs](projects/completed/log_parser_cli.rs) | Iterators + parsing + aggregation |
| 30 | ⭐ 4/10 | Planned | `qr_tool` | [qr_tool.rs](projects/planned/rating_4/qr_tool.rs) | QR encode/decode utility |
| 31 | ⭐ 4/10 | Planned | `batch_renamer` | [batch_renamer.rs](projects/planned/rating_4/batch_renamer.rs) | Safe bulk filename transformation |
| 32 | ⭐ 4/10 | Planned | `markdown_reader` | [markdown_reader.rs](projects/planned/rating_4/markdown_reader.rs) | Terminal markdown viewing |
| 33 | ⭐ 4/10 | Planned | `totp_manager` | [totp_manager.rs](projects/planned/rating_4/totp_manager.rs) | Local TOTP code generator |
| 34 | ⭐ 4/10 | Planned | `password_store_cli` | [password_store_cli.rs](projects/planned/rating_4/password_store_cli.rs) | Simple encrypted secret storage |
| 35 | ⭐ 4/10 | Planned | `text_diff_cli` | [text_diff_cli.rs](projects/planned/rating_4/text_diff_cli.rs) | Line-by-line text comparison |
| 36 | ⭐ 4/10 | Planned | `http_client_cli` | [http_client_cli.rs](projects/planned/rating_4/http_client_cli.rs) | Send simple HTTP requests |
| 37 | ⭐ 4/10 | Planned | `archive_extractor` | [archive_extractor.rs](projects/planned/rating_4/archive_extractor.rs) | List and extract simple archives |
| 38 | ⭐ 4/10 | Planned | `ini_inspector` | [ini_inspector.rs](projects/planned/rating_4/ini_inspector.rs) | Inspect and query INI files |
| 39 | ⭐ 4/10 | Planned | `date_calculator` | [date_calculator.rs](projects/planned/rating_4/date_calculator.rs) | Perform simple date arithmetic |
| 40 | ⭐ 5/10 | Planned | `rest_api` | [rest_api.rs](projects/planned/rating_5/rest_api.rs) | Production backend |
| 41 | ⭐ 5/10 | Planned | `crawler` | [web_crawler.rs](projects/planned/rating_5/web_crawler.rs) | Async mindset |
| 42 | ⭐ 5/10 | Planned | `dns_toolkit` | [dns_toolkit.rs](projects/planned/rating_5/dns_toolkit.rs) | Binary protocol + UDP networking |
| 43 | ⭐ 5/10 | Planned | `mini_git` | [mini_git.rs](projects/planned/rating_5/mini_git.rs) | Content-addressed storage |
| 44 | ⭐ 5/10 | Planned | `loadtest` | [http_load_tester.rs](projects/planned/rating_5/http_load_tester.rs) | Performance testing + latency stats |
| 45 | ⭐ 5/10 | Planned | `markdown_parser` | [markdown_parser.rs](projects/planned/rating_5/markdown_parser.rs) | Text format parsing |
| 46 | ⭐ 5/10 | Planned | `spell_checker` | [spell_checker.rs](projects/planned/rating_5/spell_checker.rs) | String processing + suggestion ranking |
| 47 | ⭐ 5/10 | Planned | `cron_scheduler` | [cron_scheduler.rs](projects/planned/rating_5/cron_scheduler.rs) | Run jobs on cron-like schedules |
| 48 | ⭐ 5/10 | Planned | `api_mock_server` | [api_mock_server.rs](projects/planned/rating_5/api_mock_server.rs) | Serve configurable mock API responses |
| 49 | ⭐ 5/10 | Planned | `notification_hub` | [notification_hub.rs](projects/planned/rating_5/notification_hub.rs) | Route notifications to multiple sinks |
| 50 | ⭐ 5/10 | Planned | `artifact_signer` | [artifact_signer.rs](projects/planned/rating_5/artifact_signer.rs) | Sign and verify build artifacts |
| 51 | ⭐ 5/10 | Planned | `quota_service` | [quota_service.rs](projects/planned/rating_5/quota_service.rs) | Enforce tenant and endpoint quotas |
| 52 | ⭐ 5/10 | Planned | `session_store` | [session_store.rs](projects/planned/rating_5/session_store.rs) | Manage expiring user sessions safely |
| 53 | ⭐ 5/10 | Planned | `synthetic_probe` | [synthetic_probe.rs](projects/planned/rating_5/synthetic_probe.rs) | Run scheduled probes and health checks |
| 54 | ⭐ 5/10 | Planned | `incident_timeline` | [incident_timeline.rs](projects/planned/rating_5/incident_timeline.rs) | Build incident timelines from event streams |
| 55 | ⭐ 5/10 | Planned | `runtime_profiler` | [runtime_profiler.rs](projects/planned/rating_5/runtime_profiler.rs) | Summarize runtime hotspots and regressions |
| 56 | ⭐ 5/10 | Planned | `incident_postmortem` | [incident_postmortem.rs](projects/planned/rating_5/incident_postmortem.rs) | Generate postmortems from incident records |
| 57 | ⭐ 5/10 | Planned | `uptime_sla_tracker` | [uptime_sla_tracker.rs](projects/planned/rating_5/uptime_sla_tracker.rs) | Measure SLA compliance and error budgets |
| 58 | ⭐ 5/10 | Planned | `api_deprecation_tracker` | [api_deprecation_tracker.rs](projects/planned/rating_5/api_deprecation_tracker.rs) | Track API sunset timelines and usage risk |
| 59 | ⭐ 5/10 | Planned | `maintenance_window_manager` | [maintenance_window_manager.rs](projects/planned/rating_5/maintenance_window_manager.rs) | Plan and validate maintenance windows |
| 60 | ⭐ 6/10 | Planned | `redis_clone` | [redis_clone.rs](projects/planned/rating_6/redis_clone.rs) | Network + memory |
| 61 | ⭐ 6/10 | Planned | `metrics_collector` | [metrics_collector.rs](projects/planned/rating_6/metrics_collector.rs) | Observability |
| 62 | ⭐ 6/10 | Planned | `job_queue` | [job_queue.rs](projects/planned/rating_6/job_queue.rs) | Retries, DLQ, worker orchestration |
| 63 | ⭐ 6/10 | Planned | `inv_index` | [inverted_index.rs](projects/planned/rating_6/inverted_index.rs) | Search indexing + ranking |
| 64 | ⭐ 6/10 | Planned | `mini_shell` | [mini_shell.rs](projects/planned/rating_6/mini_shell.rs) | Process control + pipes |
| 65 | ⭐ 6/10 | WIP | `invaders` | [invaders.rs](projects/wip/invaders.rs) | Terminal game loop + rendering |
| 66 | ⭐ 6/10 | Planned | `auth_service` | [auth_service.rs](projects/planned/rating_6/auth_service.rs) | Authentication and session design |
| 67 | ⭐ 6/10 | Planned | `wasm_game_of_life` | [wasm_game_of_life.rs](projects/planned/rating_6/wasm_game_of_life.rs) | Rust + WebAssembly UI loop |
| 68 | ⭐ 6/10 | Planned | `feature_flag_server` | [feature_flag_server.rs](projects/planned/rating_6/feature_flag_server.rs) | Rollout targeting + evaluation API |
| 69 | ⭐ 6/10 | Planned | `image_optimizer` | [image_optimizer.rs](projects/planned/rating_6/image_optimizer.rs) | Lossless compression + batch processing |
| 70 | ⭐ 6/10 | Planned | `power_monitor` | [power_monitor.rs](projects/planned/rating_6/power_monitor.rs) | Host energy sampling + trend reporting |
| 71 | ⭐ 6/10 | Planned | `rss_pipeline` | [rss_pipeline.rs](projects/planned/rating_6/rss_pipeline.rs) | Feed ingestion + transform pipeline |
| 72 | ⭐ 6/10 | Planned | `feature_flag_service` | [feature_flag_service.rs](projects/planned/rating_6/feature_flag_service.rs) | Evaluate feature flags over an API |
| 73 | ⭐ 6/10 | Planned | `backup_tool` | [backup_tool.rs](projects/planned/rating_6/backup_tool.rs) | Create and restore filesystem snapshots |
| 74 | ⭐ 6/10 | Planned | `metrics_dashboard` | [metrics_dashboard.rs](projects/planned/rating_6/metrics_dashboard.rs) | Aggregate and visualize runtime metrics |
| 75 | ⭐ 6/10 | Planned | `config_deployer` | [config_deployer.rs](projects/planned/rating_6/config_deployer.rs) | Validate and roll out configuration changes |
| 76 | ⭐ 6/10 | Planned | `webhook_dispatcher` | [webhook_dispatcher.rs](projects/planned/rating_6/webhook_dispatcher.rs) | Deliver webhooks with retry/backoff |
| 77 | ⭐ 6/10 | Planned | `incident_router` | [incident_router.rs](projects/planned/rating_6/incident_router.rs) | Route incidents with escalation policies |
| 78 | ⭐ 6/10 | Planned | `edge_cache` | [edge_cache.rs](projects/planned/rating_6/edge_cache.rs) | Cache and revalidate edge responses |
| 79 | ⭐ 6/10 | Planned | `audit_trail_store` | [audit_trail_store.rs](projects/planned/rating_6/audit_trail_store.rs) | Append-only audit event storage |
| 80 | ⭐ 6/10 | Planned | `api_contract_tester` | [api_contract_tester.rs](projects/planned/rating_6/api_contract_tester.rs) | Validate APIs against contract expectations |
| 81 | ⭐ 6/10 | Planned | `cache_invalidator` | [cache_invalidator.rs](projects/planned/rating_6/cache_invalidator.rs) | Coordinate distributed cache invalidation |
| 82 | ⭐ 6/10 | Planned | `release_train` | [release_train.rs](projects/planned/rating_6/release_train.rs) | Coordinate gated release train promotions |
| 83 | ⭐ 6/10 | Planned | `policy_simulator` | [policy_simulator.rs](projects/planned/rating_6/policy_simulator.rs) | Dry-run policy changes with explain traces |
| 84 | ⭐ 6/10 | Planned | `tenant_billing_meter` | [tenant_billing_meter.rs](projects/planned/rating_6/tenant_billing_meter.rs) | Meter tenant usage for billing pipelines |
| 85 | ⭐ 6/10 | Planned | `service_catalog` | [service_catalog.rs](projects/planned/rating_6/service_catalog.rs) | Maintain service metadata and dependencies |
| 86 | ⭐ 6/10 | Planned | `capacity_planner` | [capacity_planner.rs](projects/planned/rating_6/capacity_planner.rs) | Forecast capacity and recommend scaling actions |
| 87 | ⭐ 6/10 | Planned | `config_diff_auditor` | [config_diff_auditor.rs](projects/planned/rating_6/config_diff_auditor.rs) | Audit semantic configuration changes |
| 88 | ⭐ 6/10 | Planned | `log_redaction_gateway` | [log_redaction_gateway.rs](projects/planned/rating_6/log_redaction_gateway.rs) | Redact sensitive fields from log streams |
| 89 | ⭐ 6/10 | Planned | `incident_simulator` | [incident_simulator.rs](projects/planned/rating_6/incident_simulator.rs) | Simulate incident scenarios and response timings |
| 90 | ⭐ 6/10 | Planned | `workload_forecaster` | [workload_forecaster.rs](projects/planned/rating_6/workload_forecaster.rs) | Forecast workload demand and variance |
| 91 | ⭐ 6/10 | Planned | `credential_inventory` | [credential_inventory.rs](projects/planned/rating_6/credential_inventory.rs) | Inventory credentials and detect stale access |
| 92 | ⭐ 6/10 | Planned | `runbook_recommender` | [runbook_recommender.rs](projects/planned/rating_6/runbook_recommender.rs) | Recommend operational runbooks for incidents |
| 93 | ⭐ 6/10 | Planned | `latency_budget_planner` | [latency_budget_planner.rs](projects/planned/rating_6/latency_budget_planner.rs) | Allocate and track end-to-end latency budgets |
| 94 | ⭐ 6/10 | Planned | `dependency_risk_heatmap` | [dependency_risk_heatmap.rs](projects/planned/rating_6/dependency_risk_heatmap.rs) | Visualize dependency risk concentration |
| 95 | ⭐ 6/10 | Planned | `incident_comms_broker` | [incident_comms_broker.rs](projects/planned/rating_6/incident_comms_broker.rs) | Broker incident updates across channels |
| 96 | ⭐ 7/10 | Completed | `api_gateway` | [api_gateway.rs](projects/completed/api_gateway.rs) | Architecture |
| 97 | ⭐ 7/10 | Planned | `kafka_consumer` | [kafka_consumer.rs](projects/planned/rating_7/kafka_consumer.rs) | Enterprise patterns |
| 98 | ⭐ 7/10 | Planned | `textedit` | [text_editor.rs](projects/planned/rating_7/text_editor.rs) | Terminal UI + buffer editing |
| 99 | ⭐ 7/10 | Planned | `socks5_proxy` | [socks5_proxy.rs](projects/planned/rating_7/socks5_proxy.rs) | Protocol parsing + TCP proxying |
| 100 | ⭐ 7/10 | Planned | `ws_broker` | [websocket_broker.rs](projects/planned/rating_7/websocket_broker.rs) | Realtime pub/sub patterns |
| 101 | ⭐ 7/10 | Planned | `rate_limiter` | [rate_limiter.rs](projects/planned/rating_7/rate_limiter.rs) | Traffic shaping algorithms |
| 102 | ⭐ 7/10 | Planned | `tcp_stack` | [tcp_stack.rs](projects/planned/rating_7/tcp_stack.rs) | Network stack internals |
| 103 | ⭐ 7/10 | Planned | `mqtt_broker` | [mqtt_broker.rs](projects/planned/rating_7/mqtt_broker.rs) | Pub/sub protocol server design |
| 104 | ⭐ 7/10 | Planned | `chip8` | [chip8.rs](projects/planned/rating_7/chip8.rs) | Emulator architecture |
| 105 | ⭐ 7/10 | Planned | `ray_tracer` | [ray_tracer.rs](projects/planned/rating_7/ray_tracer.rs) | Graphics math + rendering |
| 106 | ⭐ 7/10 | Planned | `image_codec` | [image_codec.rs](projects/planned/rating_7/image_codec.rs) | Binary file formats |
| 107 | ⭐ 7/10 | Planned | `terminal_emulator` | [terminal_emulator.rs](projects/planned/rating_7/terminal_emulator.rs) | PTY handling + ANSI parsing |
| 108 | ⭐ 7/10 | Planned | `ci_system` | [ci_system.rs](projects/planned/rating_7/ci_system.rs) | Pipeline orchestration + workers |
| 109 | ⭐ 7/10 | Planned | `build_system` | [build_system.rs](projects/planned/rating_7/build_system.rs) | DAG execution + incremental builds |
| 110 | ⭐ 7/10 | Planned | `secret_scanner` | [secret_scanner.rs](projects/planned/rating_7/secret_scanner.rs) | Credential detection + risk reporting |
| 111 | ⭐ 7/10 | Planned | `policy_engine` | [policy_engine.rs](projects/planned/rating_7/policy_engine.rs) | Authorization rules + decision traces |
| 112 | ⭐ 7/10 | Planned | `spreadsheet_engine` | [spreadsheet_engine.rs](projects/planned/rating_7/spreadsheet_engine.rs) | Formula graphs + recalculation |
| 113 | ⭐ 7/10 | Planned | `webhook_gateway` | [webhook_gateway.rs](projects/planned/rating_7/webhook_gateway.rs) | Signed delivery + retry queues |
| 114 | ⭐ 7/10 | Planned | `config_manager` | [config_manager.rs](projects/planned/rating_7/config_manager.rs) | Declarative state reconciliation |
| 115 | ⭐ 7/10 | Planned | `irc_server` | [irc_server.rs](projects/planned/rating_7/irc_server.rs) | Realtime chat protocol server |
| 116 | ⭐ 7/10 | Planned | `workflow_scheduler` | [workflow_scheduler.rs](projects/planned/rating_7/workflow_scheduler.rs) | Timed triggers + durable retries |
| 117 | ⭐ 7/10 | Planned | `dns_resolver` | [dns_resolver.rs](projects/planned/rating_7/dns_resolver.rs) | Recursive lookups + TTL caching |
| 118 | ⭐ 7/10 | Planned | `ftp_server` | [ftp_server.rs](projects/planned/rating_7/ftp_server.rs) | Control/data channel protocol handling |
| 119 | ⭐ 7/10 | Planned | `release_engineering` | [release_engineering.rs](projects/planned/rating_7/release_engineering.rs) | Versioning + artifact promotion |
| 120 | ⭐ 7/10 | Planned | `dedup_engine` | [dedup_engine.rs](projects/planned/rating_7/dedup_engine.rs) | File deduplication + reclaim planning |
| 121 | ⭐ 7/10 | Planned | `crash_reporter` | [crash_reporter.rs](projects/planned/rating_7/crash_reporter.rs) | Crash event intake + issue grouping |
| 122 | ⭐ 7/10 | Planned | `bot_framework` | [bot_framework.rs](projects/planned/rating_7/bot_framework.rs) | Event-driven bot runtime + adapters |
| 123 | ⭐ 7/10 | Planned | `wiki_engine` | [wiki_engine.rs](projects/planned/rating_7/wiki_engine.rs) | Revisioned pages + internal link graph |
| 124 | ⭐ 7/10 | Planned | `torrent_tracker` | [torrent_tracker.rs](projects/planned/rating_7/torrent_tracker.rs) | Peer announce handling + swarm coordination |
| 125 | ⭐ 7/10 | Planned | `statsd_server` | [statsd_server.rs](projects/planned/rating_7/statsd_server.rs) | Metrics line ingestion + timed aggregation |
| 126 | ⭐ 7/10 | Planned | `notebook_sync_engine` | [notebook_sync_engine.rs](projects/planned/rating_7/notebook_sync_engine.rs) | Sync notebooks and detect conflicts |
| 127 | ⭐ 7/10 | Planned | `search_indexer` | [search_indexer.rs](projects/planned/rating_7/search_indexer.rs) | Build and query an inverted search index |
| 128 | ⭐ 7/10 | Planned | `media_library_server` | [media_library_server.rs](projects/planned/rating_7/media_library_server.rs) | Catalog and browse a media library |
| 129 | ⭐ 7/10 | Planned | `ci_runner` | [ci_runner.rs](projects/planned/rating_7/ci_runner.rs) | Execute CI jobs with isolated steps |
| 130 | ⭐ 7/10 | Planned | `job_queue_server` | [job_queue_server.rs](projects/planned/rating_7/job_queue_server.rs) | Manage durable queues and worker leases |
| 131 | ⭐ 7/10 | Planned | `sbom_scanner` | [sbom_scanner.rs](projects/planned/rating_7/sbom_scanner.rs) | Build SBOM reports and risk summaries |
| 132 | ⭐ 7/10 | Planned | `rollout_manager` | [rollout_manager.rs](projects/planned/rating_7/rollout_manager.rs) | Execute staged feature rollouts safely |
| 133 | ⭐ 7/10 | Planned | `dependency_mirror` | [dependency_mirror.rs](projects/planned/rating_7/dependency_mirror.rs) | Mirror dependency metadata and artifacts |
| 134 | ⭐ 7/10 | Planned | `compliance_checker` | [compliance_checker.rs](projects/planned/rating_7/compliance_checker.rs) | Evaluate resources against policy controls |
| 135 | ⭐ 7/10 | Planned | `tenant_provisioner` | [tenant_provisioner.rs](projects/planned/rating_7/tenant_provisioner.rs) | Provision tenant resources with rollback safety |
| 136 | ⭐ 7/10 | Planned | `dependency_auditor` | [dependency_auditor.rs](projects/planned/rating_7/dependency_auditor.rs) | Audit dependency graphs against policy |
| 137 | ⭐ 7/10 | Planned | `traffic_replayer` | [traffic_replayer.rs](projects/planned/rating_7/traffic_replayer.rs) | Replay production-like request traffic safely |
| 138 | ⭐ 7/10 | Planned | `access_review_engine` | [access_review_engine.rs](projects/planned/rating_7/access_review_engine.rs) | Run periodic entitlement access reviews |
| 139 | ⭐ 7/10 | Planned | `secret_rotation_service` | [secret_rotation_service.rs](projects/planned/rating_7/secret_rotation_service.rs) | Coordinate scheduled secret rotations safely |
| 140 | ⭐ 7/10 | Planned | `api_replay_harness` | [api_replay_harness.rs](projects/planned/rating_7/api_replay_harness.rs) | Replay API sessions and compare baselines |
| 141 | ⭐ 7/10 | Planned | `change_approval_engine` | [change_approval_engine.rs](projects/planned/rating_7/change_approval_engine.rs) | Evaluate and route change approvals |
| 142 | ⭐ 7/10 | Planned | `delivery_slo_guard` | [delivery_slo_guard.rs](projects/planned/rating_7/delivery_slo_guard.rs) | Guard delivery SLOs with burn-rate signals |
| 143 | ⭐ 7/10 | Planned | `blast_radius_analyzer` | [blast_radius_analyzer.rs](projects/planned/rating_7/blast_radius_analyzer.rs) | Estimate impact scope from dependency graphs |
| 144 | ⭐ 7/10 | Planned | `release_guardrail` | [release_guardrail.rs](projects/planned/rating_7/release_guardrail.rs) | Enforce release readiness guard checks |
| 145 | ⭐ 8/10 | Planned | `workflow_engine` | [workflow_engine.rs](projects/planned/rating_8/workflow_engine.rs) | State machines + DAG |
| 146 | ⭐ 8/10 | Planned | `otel_collector` | [otel_collector.rs](projects/planned/rating_8/otel_collector.rs) | Infra + streaming |
| 147 | ⭐ 8/10 | Planned | `lsm_kv` | [lsm_kv.rs](projects/planned/rating_8/lsm_kv.rs) | Storage engine internals |
| 148 | ⭐ 8/10 | Planned | `bloom_hll` | [bloom_hll.rs](projects/planned/rating_8/bloom_hll.rs) | Probabilistic data structures |
| 149 | ⭐ 8/10 | Planned | `file_sync` | [file_sync.rs](projects/planned/rating_8/file_sync.rs) | Filesystem diff + sync engine |
| 150 | ⭐ 8/10 | Planned | `template_engine` | [template_engine.rs](projects/planned/rating_8/template_engine.rs) | Parsing + AST + rendering |
| 151 | ⭐ 8/10 | Planned | `browser_engine` | [browser_engine.rs](projects/planned/rating_8/browser_engine.rs) | Parsing + layout + rendering pipeline |
| 152 | ⭐ 8/10 | Planned | `bytecode_vm` | [bytecode_vm.rs](projects/planned/rating_8/bytecode_vm.rs) | Interpreter internals |
| 153 | ⭐ 8/10 | Planned | `mini_compiler` | [mini_compiler.rs](projects/planned/rating_8/mini_compiler.rs) | Language design + codegen |
| 154 | ⭐ 8/10 | Planned | `packet_sniffer` | [packet_sniffer.rs](projects/planned/rating_8/packet_sniffer.rs) | Packet parsing + traffic analysis |
| 155 | ⭐ 8/10 | Planned | `static_site_gen` | [static_site_gen.rs](projects/planned/rating_8/static_site_gen.rs) | Build pipeline + content generation |
| 156 | ⭐ 8/10 | Planned | `toy_debugger` | [toy_debugger.rs](projects/planned/rating_8/toy_debugger.rs) | Process introspection + breakpoints |
| 157 | ⭐ 8/10 | Planned | `nes_emulator` | [nes_emulator.rs](projects/planned/rating_8/nes_emulator.rs) | Hardware emulation + timing |
| 158 | ⭐ 8/10 | Planned | `memory_allocator` | [memory_allocator.rs](projects/planned/rating_8/memory_allocator.rs) | Allocation strategies + invariants |
| 159 | ⭐ 8/10 | Planned | `physics_engine` | [physics_engine.rs](projects/planned/rating_8/physics_engine.rs) | Simulation + collision resolution |
| 160 | ⭐ 8/10 | Planned | `fuse_fs` | [fuse_fs.rs](projects/planned/rating_8/fuse_fs.rs) | Filesystem semantics + inode modeling |
| 161 | ⭐ 8/10 | Planned | `search_engine` | [search_engine.rs](projects/planned/rating_8/search_engine.rs) | Indexing + relevance ranking |
| 162 | ⭐ 8/10 | Planned | `package_manager` | [package_manager.rs](projects/planned/rating_8/package_manager.rs) | Dependency resolution + artifact lifecycle |
| 163 | ⭐ 8/10 | Planned | `message_queue` | [message_queue.rs](projects/planned/rating_8/message_queue.rs) | Durable pub/sub + consumer groups |
| 164 | ⭐ 8/10 | Planned | `smtp_server` | [smtp_server.rs](projects/planned/rating_8/smtp_server.rs) | Mail transport + delivery queue |
| 165 | ⭐ 8/10 | Planned | `backup_engine` | [backup_engine.rs](projects/planned/rating_8/backup_engine.rs) | Snapshots + dedup + restore |
| 166 | ⭐ 8/10 | Planned | `cargo_registry` | [cargo_registry.rs](projects/planned/rating_8/cargo_registry.rs) | Package index + artifact publishing |
| 167 | ⭐ 8/10 | Planned | `language_server` | [language_server.rs](projects/planned/rating_8/language_server.rs) | LSP protocol + incremental diagnostics |
| 168 | ⭐ 8/10 | Planned | `container_registry` | [container_registry.rs](projects/planned/rating_8/container_registry.rs) | OCI manifests + blob lifecycle |
| 169 | ⭐ 8/10 | Planned | `map_tile_server` | [map_tile_server.rs](projects/planned/rating_8/map_tile_server.rs) | Geospatial tile serving + cache |
| 170 | ⭐ 8/10 | Planned | `service_discovery` | [service_discovery.rs](projects/planned/rating_8/service_discovery.rs) | Dynamic registry + health watches |
| 171 | ⭐ 8/10 | Planned | `tracing_backend` | [tracing_backend.rs](projects/planned/rating_8/tracing_backend.rs) | Trace ingest + search backend |
| 172 | ⭐ 8/10 | Planned | `mailing_list_manager` | [mailing_list_manager.rs](projects/planned/rating_8/mailing_list_manager.rs) | Subscription lifecycle + moderated delivery |
| 173 | ⭐ 8/10 | Planned | `ocr_engine` | [ocr_engine.rs](projects/planned/rating_8/ocr_engine.rs) | Image preprocessing + text recognition |
| 174 | ⭐ 8/10 | Planned | `video_transcoder` | [video_transcoder.rs](projects/planned/rating_8/video_transcoder.rs) | Media job orchestration + progress tracking |
| 175 | ⭐ 8/10 | Planned | `static_analyzer` | [static_analyzer.rs](projects/planned/rating_8/static_analyzer.rs) | Rule engine + source diagnostics |
| 176 | ⭐ 8/10 | Planned | `browser_automation` | [browser_automation.rs](projects/planned/rating_8/browser_automation.rs) | Session orchestration + DOM actions |
| 177 | ⭐ 8/10 | Planned | `schema_registry` | [schema_registry.rs](projects/planned/rating_8/schema_registry.rs) | Schema evolution + compatibility checks |
| 178 | ⭐ 8/10 | Planned | `secrets_manager` | [secrets_manager.rs](projects/planned/rating_8/secrets_manager.rs) | Encrypted secret storage + policy control |
| 179 | ⭐ 8/10 | Planned | `sip_proxy` | [sip_proxy.rs](projects/planned/rating_8/sip_proxy.rs) | VoIP signaling routing + registration |
| 180 | ⭐ 8/10 | Planned | `reverse_tunnel` | [reverse_tunnel.rs](projects/planned/rating_8/reverse_tunnel.rs) | Secure port exposure + multiplexed forwarding |
| 181 | ⭐ 8/10 | Planned | `live_stream_server` | [live_stream_server.rs](projects/planned/rating_8/live_stream_server.rs) | Live ingest + segment serving pipeline |
| 182 | ⭐ 8/10 | Planned | `imap_server` | [imap_server.rs](projects/planned/rating_8/imap_server.rs) | Mailbox protocol server + sync semantics |
| 183 | ⭐ 8/10 | Planned | `modbus_gateway` | [modbus_gateway.rs](projects/planned/rating_8/modbus_gateway.rs) | Industrial protocol bridge + polling |
| 184 | ⭐ 8/10 | Planned | `graphql_gateway` | [graphql_gateway.rs](projects/planned/rating_8/graphql_gateway.rs) | Schema composition + resolver orchestration |
| 185 | ⭐ 8/10 | Planned | `terminal_multiplexer` | [terminal_multiplexer.rs](projects/planned/rating_8/terminal_multiplexer.rs) | PTY sessions + pane layout control |
| 186 | ⭐ 8/10 | Planned | `ldap_server` | [ldap_server.rs](projects/planned/rating_8/ldap_server.rs) | Directory service + LDAP query handling |
| 187 | ⭐ 8/10 | Planned | `mail_archive` | [mail_archive.rs](projects/planned/rating_8/mail_archive.rs) | MIME archival + searchable retention |
| 188 | ⭐ 8/10 | Planned | `artifact_proxy` | [artifact_proxy.rs](projects/planned/rating_8/artifact_proxy.rs) | Upstream package proxy + integrity cache |
| 189 | ⭐ 8/10 | Planned | `dns_authority` | [dns_authority.rs](projects/planned/rating_8/dns_authority.rs) | Authoritative zone serving + delegation rules |
| 190 | ⭐ 8/10 | Planned | `package_registry` | [package_registry.rs](projects/planned/rating_8/package_registry.rs) | Publish and fetch package artifacts |
| 191 | ⭐ 8/10 | Planned | `plugin_runtime` | [plugin_runtime.rs](projects/planned/rating_8/plugin_runtime.rs) | Load and run sandboxed plugins |
| 192 | ⭐ 8/10 | Planned | `canary_controller` | [canary_controller.rs](projects/planned/rating_8/canary_controller.rs) | Automate canary promotion and rollback |
| 193 | ⭐ 8/10 | Planned | `chaos_orchestrator` | [chaos_orchestrator.rs](projects/planned/rating_8/chaos_orchestrator.rs) | Run controlled fault injection experiments |
| 194 | ⭐ 8/10 | Planned | `threat_feed_aggregator` | [threat_feed_aggregator.rs](projects/planned/rating_8/threat_feed_aggregator.rs) | Aggregate and score threat intelligence feeds |
| 195 | ⭐ 8/10 | Planned | `failover_coordinator` | [failover_coordinator.rs](projects/planned/rating_8/failover_coordinator.rs) | Orchestrate failover and safe failback decisions |
| 196 | ⭐ 8/10 | Planned | `schema_migration_planner` | [schema_migration_planner.rs](projects/planned/rating_8/schema_migration_planner.rs) | Plan safe schema migrations with rollbacks |
| 197 | ⭐ 8/10 | Planned | `authz_drift_detector` | [authz_drift_detector.rs](projects/planned/rating_8/authz_drift_detector.rs) | Detect authorization drift from desired state |
| 198 | ⭐ 8/10 | WIP | `rust_engine_game` | [rust_engine_game.rs](projects/wip/rust_engine_game.rs) | 2D game loop + collision + scoring |
| 199 | ⭐ 9/10 | Planned | `fake_sql_server` | [fake_sql_server.rs](projects/planned/rating_9/fake_sql_server.rs) | Protocol engineering |
| 200 | ⭐ 9/10 | Planned | `sidecar` | [service_mesh_sidecar.rs](projects/planned/rating_9/service_mesh_sidecar.rs) | Networking mastery |
| 201 | ⭐ 9/10 | Planned | `regex_engine` | [regex_engine.rs](projects/planned/rating_9/regex_engine.rs) | Automata theory in practice |
| 202 | ⭐ 9/10 | Planned | `bittorrent` | [bittorrent.rs](projects/planned/rating_9/bittorrent.rs) | P2P protocols + piece scheduling |
| 203 | ⭐ 9/10 | Planned | `iot_simulator` | [iot_simulator.rs](projects/planned/rating_9/iot_simulator.rs) | Distributed device simulation |
| 204 | ⭐ 9/10 | Planned | `sqlite_clone` | [sqlite_clone.rs](projects/planned/rating_9/sqlite_clone.rs) | SQL engine + B-tree storage |
| 205 | ⭐ 9/10 | Planned | `wasm_runtime` | [wasm_runtime.rs](projects/planned/rating_9/wasm_runtime.rs) | WebAssembly decoding + execution |
| 206 | ⭐ 9/10 | Planned | `container_runtime` | [container_runtime.rs](projects/planned/rating_9/container_runtime.rs) | Isolation primitives + resource control |
| 207 | ⭐ 9/10 | Planned | `vpn_tunnel` | [vpn_tunnel.rs](projects/planned/rating_9/vpn_tunnel.rs) | Secure tunneling + peer state |
| 208 | ⭐ 9/10 | Planned | `graph_database` | [graph_database.rs](projects/planned/rating_9/graph_database.rs) | Property graph storage + query execution |
| 209 | ⭐ 9/10 | Planned | `ebpf_monitor` | [ebpf_monitor.rs](projects/planned/rating_9/ebpf_monitor.rs) | Kernel telemetry pipeline design |
| 210 | ⭐ 9/10 | Planned | `blockchain_node` | [blockchain_node.rs](projects/planned/rating_9/blockchain_node.rs) | Ledger validation + peer protocol |
| 211 | ⭐ 9/10 | Planned | `object_store` | [object_store.rs](projects/planned/rating_9/object_store.rs) | Blob storage + metadata integrity |
| 212 | ⭐ 9/10 | Planned | `coord_service` | [coord_service.rs](projects/planned/rating_9/coord_service.rs) | Coordination primitives + watches |
| 213 | ⭐ 9/10 | Planned | `stream_processor` | [stream_processor.rs](projects/planned/rating_9/stream_processor.rs) | Stateful event windows + checkpoints |
| 214 | ⭐ 9/10 | Planned | `time_series_db` | [time_series_db.rs](projects/planned/rating_9/time_series_db.rs) | Time-series ingestion + query engine |
| 215 | ⭐ 9/10 | Planned | `vector_search_engine` | [vector_search_engine.rs](projects/planned/rating_9/vector_search_engine.rs) | Approximate nearest-neighbor indexing |
| 216 | ⭐ 9/10 | Planned | `collaborative_editor` | [collaborative_editor.rs](projects/planned/rating_9/collaborative_editor.rs) | Shared editing + convergence rules |
| 217 | ⭐ 9/10 | Planned | `event_sourcing_db` | [event_sourcing_db.rs](projects/planned/rating_9/event_sourcing_db.rs) | Append-only streams + projections |
| 218 | ⭐ 9/10 | Planned | `activitypub_server` | [activitypub_server.rs](projects/planned/rating_9/activitypub_server.rs) | Federated social protocol server |
| 219 | ⭐ 9/10 | Planned | `fuzzer_engine` | [fuzzer_engine.rs](projects/planned/rating_9/fuzzer_engine.rs) | Corpus mutation + feedback-guided execution |
| 220 | ⭐ 9/10 | Planned | `code_sandbox` | [code_sandbox.rs](projects/planned/rating_9/code_sandbox.rs) | Policy-driven isolated execution |
| 221 | ⭐ 10/10 | Planned | `dist_cache` | [dist_cache.rs](projects/planned/rating_10/dist_cache.rs) | Rust architect level |
| 222 | ⭐ 10/10 | Planned | `mini_runtime` | [mini_runtime.rs](projects/planned/rating_10/mini_runtime.rs) | Deep Rust internals |
| 223 | ⭐ 10/10 | Planned | `raft_consensus` | [raft_consensus.rs](projects/planned/rating_10/raft_consensus.rs) | Consensus algorithm implementation |
| 224 | ⭐ 10/10 | Planned | `os_kernel` | [os_kernel.rs](projects/planned/rating_10/os_kernel.rs) | Bare-metal systems programming |
| 225 | ⭐ 10/10 | Planned | `hdfs_clone` | [hdfs_clone.rs](projects/planned/rating_10/hdfs_clone.rs) | Namenode metadata + replicated blocks |
| 226 | ⭐ 1/10 | WIP | `sorter` | [line_sorter.rs](projects/wip/line_sorter.rs) | Sort and filter file lines |
| 227 | ⭐ 1/10 | WIP | `concat` | [file_concatenator.rs](projects/wip/file_concatenator.rs) | Combine multiple files |
| 228 | ⭐ 2/10 | WIP | `case_convert` | [string_case_converter.rs](projects/wip/string_case_converter.rs) | Convert between naming conventions |
| 229 | ⭐ 2/10 | WIP | `wordfreq` | [word_frequency_counter.rs](projects/wip/word_frequency_counter.rs) | Count word occurrences in text |
| 230 | ⭐ 2/10 | WIP | `txtstat` | [text_statistics.rs](projects/wip/text_statistics.rs) | Count lines, words, characters |
| 231 | ⭐ 2/10 | WIP | `grepish` | [simple_grep.rs](projects/wip/simple_grep.rs) | Basic text search tool |
| 232 | ⭐ 2/10 | WIP | `number_guess` | [number_guessing_game.rs](projects/wip/number_guessing_game.rs) | Interactive guessing game |
| 233 | ⭐ 3/10 | WIP | `todocli` | [simple_todo_app.rs](projects/wip/simple_todo_app.rs) | CLI task manager with persistence |

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

- [dns_toolkit_TODO.md](projects/planned/rating_5/dns_toolkit_TODO.md)

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

- [mini_git_TODO.md](projects/planned/rating_5/mini_git_TODO.md)

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

- [http_load_tester_TODO.md](projects/planned/rating_5/http_load_tester_TODO.md)

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

- [job_queue_TODO.md](projects/planned/rating_6/job_queue_TODO.md)

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

- [inverted_index_TODO.md](projects/planned/rating_6/inverted_index_TODO.md)

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

- [mini_shell_TODO.md](projects/planned/rating_6/mini_shell_TODO.md)

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

- [text_editor_TODO.md](projects/planned/rating_7/text_editor_TODO.md)

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

- [socks5_proxy_TODO.md](projects/planned/rating_7/socks5_proxy_TODO.md)

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

- [websocket_broker_TODO.md](projects/planned/rating_7/websocket_broker_TODO.md)

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

- [rate_limiter_TODO.md](projects/planned/rating_7/rate_limiter_TODO.md)

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

- [lsm_kv_TODO.md](projects/planned/rating_8/lsm_kv_TODO.md)

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

- [bloom_hll_TODO.md](projects/planned/rating_8/bloom_hll_TODO.md)

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

- [file_sync_TODO.md](projects/planned/rating_8/file_sync_TODO.md)

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

- [template_engine_TODO.md](projects/planned/rating_8/template_engine_TODO.md)

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

- [regex_engine_TODO.md](projects/planned/rating_9/regex_engine_TODO.md)

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

- [bittorrent_TODO.md](projects/planned/rating_9/bittorrent_TODO.md)

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

- [tcp_stack_TODO.md](projects/planned/rating_7/tcp_stack_TODO.md)

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

- [mqtt_broker_TODO.md](projects/planned/rating_7/mqtt_broker_TODO.md)

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

- [chip8_TODO.md](projects/planned/rating_7/chip8_TODO.md)

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

- [browser_engine_TODO.md](projects/planned/rating_8/browser_engine_TODO.md)

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

- [bytecode_vm_TODO.md](projects/planned/rating_8/bytecode_vm_TODO.md)

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

- [mini_compiler_TODO.md](projects/planned/rating_8/mini_compiler_TODO.md)

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

- [packet_sniffer_TODO.md](projects/planned/rating_8/packet_sniffer_TODO.md)

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

- [static_site_gen_TODO.md](projects/planned/rating_8/static_site_gen_TODO.md)

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

- [iot_simulator_TODO.md](projects/planned/rating_9/iot_simulator_TODO.md)

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

- [raft_consensus_TODO.md](projects/planned/rating_10/raft_consensus_TODO.md)

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

- [markdown_parser_TODO.md](projects/planned/rating_5/markdown_parser_TODO.md)

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

- [auth_service_TODO.md](projects/planned/rating_6/auth_service_TODO.md)

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

- [ray_tracer_TODO.md](projects/planned/rating_7/ray_tracer_TODO.md)

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

- [image_codec_TODO.md](projects/planned/rating_7/image_codec_TODO.md)

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

- [toy_debugger_TODO.md](projects/planned/rating_8/toy_debugger_TODO.md)

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

- [sqlite_clone_TODO.md](projects/planned/rating_9/sqlite_clone_TODO.md)

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

- [wasm_runtime_TODO.md](projects/planned/rating_9/wasm_runtime_TODO.md)

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

- [container_runtime_TODO.md](projects/planned/rating_9/container_runtime_TODO.md)

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

- [wasm_game_of_life_TODO.md](projects/planned/rating_6/wasm_game_of_life_TODO.md)

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

- [spell_checker_TODO.md](projects/planned/rating_5/spell_checker_TODO.md)

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

- [terminal_emulator_TODO.md](projects/planned/rating_7/terminal_emulator_TODO.md)

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

- [nes_emulator_TODO.md](projects/planned/rating_8/nes_emulator_TODO.md)

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

- [memory_allocator_TODO.md](projects/planned/rating_8/memory_allocator_TODO.md)

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

- [physics_engine_TODO.md](projects/planned/rating_8/physics_engine_TODO.md)

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

- [fuse_fs_TODO.md](projects/planned/rating_8/fuse_fs_TODO.md)

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

- [os_kernel_TODO.md](projects/planned/rating_10/os_kernel_TODO.md)

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

- [ci_system_TODO.md](projects/planned/rating_7/ci_system_TODO.md)

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

- [vpn_tunnel_TODO.md](projects/planned/rating_9/vpn_tunnel_TODO.md)

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

- [search_engine_TODO.md](projects/planned/rating_8/search_engine_TODO.md)

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

- [package_manager_TODO.md](projects/planned/rating_8/package_manager_TODO.md)

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

- [graph_database_TODO.md](projects/planned/rating_9/graph_database_TODO.md)

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

- [invaders_TODO.md](projects/wip/invaders_TODO.md)

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

- [message_queue_TODO.md](projects/planned/rating_8/message_queue_TODO.md)

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

- [build_system_TODO.md](projects/planned/rating_7/build_system_TODO.md)

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

- [secret_scanner_TODO.md](projects/planned/rating_7/secret_scanner_TODO.md)

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

- [ebpf_monitor_TODO.md](projects/planned/rating_9/ebpf_monitor_TODO.md)

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

- [blockchain_node_TODO.md](projects/planned/rating_9/blockchain_node_TODO.md)

---

## ⭐ 9/10 — Object Store

Goal: Blob storage + metadata integrity

Build:

```bash
cargo run --bin object_store
```

Learn:

- object metadata and content-addressed blob layouts
- streaming uploads and multipart assembly
- checksum validation and recovery paths
- blob lifecycle and garbage collection

Guide:

- [object_store_TODO.md](projects/planned/rating_9/object_store_TODO.md)

---

## ⭐ 8/10 — SMTP Server

Goal: Mail transport + delivery queue

Build:

```bash
cargo run --bin smtp_server
```

Learn:

- SMTP session state machines
- command parsing and envelope sequencing
- queued delivery and retry scheduling
- protocol hardening around malformed input

Guide:

- [smtp_server_TODO.md](projects/planned/rating_8/smtp_server_TODO.md)

---

## ⭐ 9/10 — Coordination Service

Goal: Coordination primitives + watches

Build:

```bash
cargo run --bin coord_service
```

Learn:

- leases and ephemeral session-bound state
- watch delivery and change notification ordering
- distributed locks and leader-election semantics
- snapshot and recovery constraints

Guide:

- [coord_service_TODO.md](projects/planned/rating_9/coord_service_TODO.md)

---

## ⭐ 7/10 — Policy Engine

Goal: Authorization rules + decision traces

Build:

```bash
cargo run --bin policy_engine
```

Learn:

- policy language and AST design
- allow/deny precedence rules
- contextual evaluation over subject/resource/action
- explainable traces for audits and debugging

Guide:

- [policy_engine_TODO.md](projects/planned/rating_7/policy_engine_TODO.md)

---

## ⭐ 9/10 — Stream Processor

Goal: Stateful event windows + checkpoints

Build:

```bash
cargo run --bin stream_processor
```

Learn:

- operator graph execution over event streams
- keyed state and window aggregation logic
- event-time progress and late-event handling
- replay-safe checkpoint and restore design

Guide:

- [stream_processor_TODO.md](projects/planned/rating_9/stream_processor_TODO.md)

---

## ⭐ 8/10 — Backup Engine

Goal: Snapshots + dedup + restore

Build:

```bash
cargo run --bin backup_engine
```

Learn:

- snapshot manifest design
- chunking and deduplicated storage
- restore correctness and retention policy
- interrupted-run recovery handling

Guide:

- [backup_engine_TODO.md](projects/planned/rating_8/backup_engine_TODO.md)

---

## ⭐ 8/10 — Cargo Registry

Goal: Package index + artifact publishing

Build:

```bash
cargo run --bin cargo_registry
```

Learn:

- crate index consistency rules
- publish flows and version conflict handling
- artifact checksum verification
- proxy/cache behavior for package sources

Guide:

- [cargo_registry_TODO.md](projects/planned/rating_8/cargo_registry_TODO.md)

---

## ⭐ 7/10 — Spreadsheet Engine

Goal: Formula graphs + recalculation

Build:

```bash
cargo run --bin spreadsheet_engine
```

Learn:

- formula parsing and typed cell values
- dependency graph maintenance
- deterministic recalculation scheduling
- cycle detection and error propagation

Guide:

- [spreadsheet_engine_TODO.md](projects/planned/rating_7/spreadsheet_engine_TODO.md)

---

## ⭐ 8/10 — Language Server

Goal: LSP protocol + incremental diagnostics

Build:

```bash
cargo run --bin language_server
```

Learn:

- JSON-RPC framing and request dispatch
- document lifecycle and incremental sync
- parser-driven diagnostics and editor features
- protocol error handling and capability negotiation

Guide:

- [language_server_TODO.md](projects/planned/rating_8/language_server_TODO.md)

---

## ⭐ 7/10 — Webhook Gateway

Goal: Signed delivery + retry queues

Build:

```bash
cargo run --bin webhook_gateway
```

Learn:

- outbound delivery models and retry strategy
- request signing and verification flows
- dead-letter handling and observability
- idempotent redelivery semantics

Guide:

- [webhook_gateway_TODO.md](projects/planned/rating_7/webhook_gateway_TODO.md)

---

## ⭐ 9/10 — Time-Series DB

Goal: Time-series ingestion + query engine

Build:

```bash
cargo run --bin time_series_db
```

Learn:

- series indexing and append paths
- retention, compaction, and block compression
- range queries and aggregate execution
- write-vs-query storage tradeoffs

Guide:

- [time_series_db_TODO.md](projects/planned/rating_9/time_series_db_TODO.md)

---

## ⭐ 7/10 — Config Manager

Goal: Declarative state reconciliation

Build:

```bash
cargo run --bin config_manager
```

Learn:

- desired-vs-actual state diffing
- idempotent apply operations
- dry-run planning and reconciliation reporting
- drift detection and failure summaries

Guide:

- [config_manager_TODO.md](projects/planned/rating_7/config_manager_TODO.md)

---

## ⭐ 8/10 — Container Registry

Goal: OCI manifests + blob lifecycle

Build:

```bash
cargo run --bin container_registry
```

Learn:

- digest-validated blob storage
- manifest and tag consistency rules
- registry auth and namespace semantics
- garbage collection for unreferenced blobs

Guide:

- [container_registry_TODO.md](projects/planned/rating_8/container_registry_TODO.md)

---

## ⭐ 7/10 — IRC Server

Goal: Realtime chat protocol server

Build:

```bash
cargo run --bin irc_server
```

Learn:

- line-oriented protocol parsing
- connection registration and nickname state
- channel membership and broadcast fanout
- disconnect cleanup and collision handling

Guide:

- [irc_server_TODO.md](projects/planned/rating_7/irc_server_TODO.md)

---

## ⭐ 6/10 — Feature Flag Server

Goal: Rollout targeting + evaluation API

Build:

```bash
cargo run --bin feature_flag_server
```

Learn:

- targeting rule evaluation
- percentage rollout bucketing
- stable hashing and deterministic assignment
- auditability for rule changes

Guide:

- [feature_flag_server_TODO.md](projects/planned/rating_6/feature_flag_server_TODO.md)

---

## ⭐ 8/10 — Map Tile Server

Goal: Geospatial tile serving + cache

Build:

```bash
cargo run --bin map_tile_server
```

Learn:

- z/x/y addressing and bounds validation
- tile-source abstraction and HTTP serving
- cache-key design for map tiles
- metadata endpoints for source introspection

Guide:

- [map_tile_server_TODO.md](projects/planned/rating_8/map_tile_server_TODO.md)

---

## ⭐ 8/10 — Service Discovery

Goal: Dynamic registry + health watches

Build:

```bash
cargo run --bin service_discovery
```

Learn:

- instance registration and heartbeat semantics
- TTL expiry and unhealthy instance eviction
- service query APIs and tag filtering
- watch delivery for changing registry state

Guide:

- [service_discovery_TODO.md](projects/planned/rating_8/service_discovery_TODO.md)

---

## ⭐ 8/10 — Tracing Backend

Goal: Trace ingest + search backend

Build:

```bash
cargo run --bin tracing_backend
```

Learn:

- span/event ingestion and trace assembly
- out-of-order span handling
- query indexes by service and time range
- retention and storage compaction tradeoffs

Guide:

- [tracing_backend_TODO.md](projects/planned/rating_8/tracing_backend_TODO.md)

---

## ⭐ 9/10 — Vector Search Engine

Goal: Approximate nearest-neighbor indexing

Build:

```bash
cargo run --bin vector_search_engine
```

Learn:

- embedding storage and collection metadata
- exact versus approximate nearest-neighbor search
- ANN index design such as HNSW or IVF
- metadata filters alongside similarity ranking

Guide:

- [vector_search_engine_TODO.md](projects/planned/rating_9/vector_search_engine_TODO.md)

---

## ⭐ 6/10 — Image Optimizer

Goal: Lossless compression + batch processing

Build:

```bash
cargo run --bin image_optimizer
```

Learn:

- image decode/encode pipeline separation
- format-specific optimization passes
- dry-run reporting and batch traversal
- metadata preservation versus stripping choices

Guide:

- [image_optimizer_TODO.md](projects/planned/rating_6/image_optimizer_TODO.md)

---

## ⭐ 7/10 — Workflow Scheduler

Goal: Timed triggers + durable retries

Build:

```bash
cargo run --bin workflow_scheduler
```

Learn:

- cron-like or interval trigger calculation
- durable due-run queueing
- retry/backoff for scheduled jobs
- restart recovery and missed-run semantics

Guide:

- [workflow_scheduler_TODO.md](projects/planned/rating_7/workflow_scheduler_TODO.md)

---

## ⭐ 9/10 — Collaborative Editor

Goal: Shared editing + convergence rules

Build:

```bash
cargo run --bin collaborative_editor
```

Learn:

- document sessions and edit operations
- CRDT or operational transform semantics
- concurrent edit convergence guarantees
- presence/cursor state versus document state

Guide:

- [collaborative_editor_TODO.md](projects/planned/rating_9/collaborative_editor_TODO.md)

---

## ⭐ 8/10 — Mailing List Manager

Goal: Subscription lifecycle + moderated delivery

Build:

```bash
cargo run --bin mailing_list_manager
```

Learn:

- subscribe/unsubscribe confirmation flows
- moderated posting pipelines
- list fanout and bounce suppression rules
- auditable membership and delivery state

Guide:

- [mailing_list_manager_TODO.md](projects/planned/rating_8/mailing_list_manager_TODO.md)

---

## ⭐ 8/10 — OCR Engine

Goal: Image preprocessing + text recognition

Build:

```bash
cargo run --bin ocr_engine
```

Learn:

- thresholding and segmentation preprocessing
- glyph extraction and recognition pipeline
- confidence scoring and error reporting
- fixture-driven evaluation against expected text

Guide:

- [ocr_engine_TODO.md](projects/planned/rating_8/ocr_engine_TODO.md)

---

## ⭐ 7/10 — DNS Resolver

Goal: Recursive lookups + TTL caching

Build:

```bash
cargo run --bin dns_resolver
```

Learn:

- DNS packet parsing and serialization
- recursive resolution flow and retries
- cache entries with TTL expiration
- record handling and NXDOMAIN behavior

Guide:

- [dns_resolver_TODO.md](projects/planned/rating_7/dns_resolver_TODO.md)

---

## ⭐ 7/10 — FTP Server

Goal: Control/data channel protocol handling

Build:

```bash
cargo run --bin ftp_server
```

Learn:

- command parsing and session state management
- passive-mode data channel lifecycle
- file listing, upload, and retrieval flows
- root-path safety and permission failures

Guide:

- [ftp_server_TODO.md](projects/planned/rating_7/ftp_server_TODO.md)

---

## ⭐ 8/10 — Video Transcoder

Goal: Media job orchestration + progress tracking

Build:

```bash
cargo run --bin video_transcoder
```

Learn:

- media probe and stream metadata planning
- transcode job execution pipelines
- progress accounting and user feedback
- cleanup and failure recovery for temporary outputs

Guide:

- [video_transcoder_TODO.md](projects/planned/rating_8/video_transcoder_TODO.md)

---

## ⭐ 8/10 — Static Analyzer

Goal: Rule engine + source diagnostics

Build:

```bash
cargo run --bin static_analyzer
```

Learn:

- syntax or semantic model construction
- diagnostic rule evaluation
- severity, suppression, and baseline handling
- machine-readable output stability

Guide:

- [static_analyzer_TODO.md](projects/planned/rating_8/static_analyzer_TODO.md)

---

## ⭐ 7/10 — Release Engineering

Goal: Versioning + artifact promotion

Build:

```bash
cargo run --bin release_engineering
```

Learn:

- semver bump logic and changelog generation
- artifact build matrix modeling
- signed release manifest production
- release candidate versus final promotion states

Guide:

- [release_engineering_TODO.md](projects/planned/rating_7/release_engineering_TODO.md)

---

## ⭐ 9/10 — Event Sourcing DB

Goal: Append-only streams + projections

Build:

```bash
cargo run --bin event_sourcing_db
```

Learn:

- append-only event stream design
- optimistic concurrency on stream writes
- snapshot and rehydration strategies
- projection rebuild and read-model maintenance

Guide:

- [event_sourcing_db_TODO.md](projects/planned/rating_9/event_sourcing_db_TODO.md)

---

## ⭐ 8/10 — Browser Automation

Goal: Session orchestration + DOM actions

Build:

```bash
cargo run --bin browser_automation
```

Learn:

- browser session and page abstraction design
- DOM query and interaction primitives
- wait conditions and transient retry handling
- diagnostics through screenshots or trace capture

Guide:

- [browser_automation_TODO.md](projects/planned/rating_8/browser_automation_TODO.md)

---

## ⭐ 10/10 — HDFS Clone

Goal: Namenode metadata + replicated blocks

Build:

```bash
cargo run --bin hdfs_clone
```

Learn:

- namenode metadata ownership model
- block splitting and datanode placement
- heartbeat, block reports, and repair behavior
- recovery from node failure and under-replication

Guide:

- [hdfs_clone_TODO.md](projects/planned/rating_10/hdfs_clone_TODO.md)

---

## ⭐ 8/10 — Schema Registry

Goal: Schema evolution + compatibility checks

Build:

```bash
cargo run --bin schema_registry
```

Learn:

- subject/version compatibility policy design
- descriptor storage and global identifier mapping
- schema registration and lookup APIs
- auditability for schema changes

Guide:

- [schema_registry_TODO.md](projects/planned/rating_8/schema_registry_TODO.md)

---

## ⭐ 8/10 — Secrets Manager

Goal: Encrypted secret storage + policy control

Build:

```bash
cargo run --bin secrets_manager
```

Learn:

- encrypted secret versioning boundaries
- least-privilege policy evaluation
- key rotation and lease semantics
- auditable access logging

Guide:

- [secrets_manager_TODO.md](projects/planned/rating_8/secrets_manager_TODO.md)

---

## ⭐ 9/10 — ActivityPub Server

Goal: Federated social protocol server

Build:

```bash
cargo run --bin activitypub_server
```

Learn:

- actor, inbox, and outbox state modeling
- signed federation delivery and verification
- idempotent inbox processing rules
- local versus remote actor data handling

Guide:

- [activitypub_server_TODO.md](projects/planned/rating_9/activitypub_server_TODO.md)

---

## ⭐ 8/10 — SIP Proxy

Goal: VoIP signaling routing + registration

Build:

```bash
cargo run --bin sip_proxy
```

Learn:

- SIP message parsing and normalization
- registration binding lifecycle
- transaction state and response routing
- digest authentication and refresh handling

Guide:

- [sip_proxy_TODO.md](projects/planned/rating_8/sip_proxy_TODO.md)

---

## ⭐ 8/10 — Reverse Tunnel

Goal: Secure port exposure + multiplexed forwarding

Build:

```bash
cargo run --bin reverse_tunnel
```

Learn:

- client/server handshake and session ownership
- remote port reservation rules
- multiplexed stream forwarding design
- reconnect and resume semantics

Guide:

- [reverse_tunnel_TODO.md](projects/planned/rating_8/reverse_tunnel_TODO.md)

---

## ⭐ 7/10 — Dedup Engine

Goal: File deduplication + reclaim planning

Build:

```bash
cargo run --bin dedup_engine
```

Learn:

- file discovery and hashing stages
- duplicate grouping and reclaim reporting
- safe replacement planning with dry-run support
- whole-file versus chunk-level dedup tradeoffs

Guide:

- [dedup_engine_TODO.md](projects/planned/rating_7/dedup_engine_TODO.md)

---

## ⭐ 8/10 — Live Stream Server

Goal: Live ingest + segment serving pipeline

Build:

```bash
cargo run --bin live_stream_server
```

Learn:

- ingest session and stream-key modeling
- segment and playlist packaging flow
- viewer delivery and retention behavior
- stream lifecycle cleanup rules

Guide:

- [live_stream_server_TODO.md](projects/planned/rating_8/live_stream_server_TODO.md)

---

## ⭐ 7/10 — Crash Reporter

Goal: Crash event intake + issue grouping

Build:

```bash
cargo run --bin crash_reporter
```

Learn:

- crash event schema and release metadata
- symbolization or source map lookup boundaries
- fingerprinting and issue grouping rules
- regression detection and retention tradeoffs

Guide:

- [crash_reporter_TODO.md](projects/planned/rating_7/crash_reporter_TODO.md)

---

## ⭐ 8/10 — IMAP Server

Goal: Mailbox protocol server + sync semantics

Build:

```bash
cargo run --bin imap_server
```

Learn:

- mailbox and UID state modeling
- IMAP command parsing and execution
- flag mutation and mailbox metadata rules
- sync semantics for concurrent sessions

Guide:

- [imap_server_TODO.md](projects/planned/rating_8/imap_server_TODO.md)

---

## ⭐ 7/10 — Bot Framework

Goal: Event-driven bot runtime + adapters

Build:

```bash
cargo run --bin bot_framework
```

Learn:

- event routing and command dispatch
- provider adapter isolation
- middleware sequencing and retries
- conversation state lifecycle

Guide:

- [bot_framework_TODO.md](projects/planned/rating_7/bot_framework_TODO.md)

---

## ⭐ 7/10 — Wiki Engine

Goal: Revisioned pages + internal link graph

Build:

```bash
cargo run --bin wiki_engine
```

Learn:

- page revision storage models
- link parsing and backlink derivation
- edit conflict detection
- search indexing for content and titles

Guide:

- [wiki_engine_TODO.md](projects/planned/rating_7/wiki_engine_TODO.md)

---

## ⭐ 8/10 — Modbus Gateway

Goal: Industrial protocol bridge + polling

Build:

```bash
cargo run --bin modbus_gateway
```

Learn:

- Modbus frame parsing and dispatch
- register modeling for industrial data
- polling, retry, and timeout orchestration
- protocol bridge design for downstream devices

Guide:

- [modbus_gateway_TODO.md](projects/planned/rating_8/modbus_gateway_TODO.md)

---

## ⭐ 8/10 — GraphQL Gateway

Goal: Schema composition + resolver orchestration

Build:

```bash
cargo run --bin graphql_gateway
```

Learn:

- schema composition and validation rules
- execution planning and resolver batching
- auth context propagation
- persisted-query and cache tradeoffs

Guide:

- [graphql_gateway_TODO.md](projects/planned/rating_8/graphql_gateway_TODO.md)

---

## ⭐ 8/10 — Terminal Multiplexer

Goal: PTY sessions + pane layout control

Build:

```bash
cargo run --bin terminal_multiplexer
```

Learn:

- PTY-backed pane lifecycle management
- split layout math and resize propagation
- input routing and focus semantics
- detach/reattach session behavior

Guide:

- [terminal_multiplexer_TODO.md](projects/planned/rating_8/terminal_multiplexer_TODO.md)

---

## ⭐ 6/10 — Power Monitor

Goal: Host energy sampling + trend reporting

Build:

```bash
cargo run --bin power_monitor
```

Learn:

- cross-platform power metric collection boundaries
- rolling aggregation over sampled data
- threshold alerting and summary reporting
- handling partial or missing metrics gracefully

Guide:

- [power_monitor_TODO.md](projects/planned/rating_6/power_monitor_TODO.md)

---

## ⭐ 7/10 — Torrent Tracker

Goal: Peer announce handling + swarm coordination

Build:

```bash
cargo run --bin torrent_tracker
```

Learn:

- announce request parsing and peer identity
- swarm membership lifecycle rules
- compact peer response encoding
- scrape metrics and stale-peer eviction

Guide:

- [torrent_tracker_TODO.md](projects/planned/rating_7/torrent_tracker_TODO.md)

---

## ⭐ 8/10 — LDAP Server

Goal: Directory service + LDAP query handling

Build:

```bash
cargo run --bin ldap_server
```

Learn:

- DN and attribute schema modeling
- bind and search protocol handling
- filter parsing and subtree matching
- access control and directory mutation rules

Guide:

- [ldap_server_TODO.md](projects/planned/rating_8/ldap_server_TODO.md)

---

## ⭐ 7/10 — StatsD Server

Goal: Metrics line ingestion + timed aggregation

Build:

```bash
cargo run --bin statsd_server
```

Learn:

- StatsD line parsing and metric typing
- aggregation windows and flush intervals
- sample-rate correction behavior
- exporter boundaries for downstream sinks

Guide:

- [statsd_server_TODO.md](projects/planned/rating_7/statsd_server_TODO.md)

---

## ⭐ 8/10 — Mail Archive

Goal: MIME archival + searchable retention

Build:

```bash
cargo run --bin mail_archive
```

Learn:

- raw email preservation and MIME parsing
- metadata extraction and indexing
- retention and legal-hold style policies
- query surfaces across headers and bodies

Guide:

- [mail_archive_TODO.md](projects/planned/rating_8/mail_archive_TODO.md)

---

## ⭐ 9/10 — Fuzzer Engine

Goal: Corpus mutation + feedback-guided execution

Build:

```bash
cargo run --bin fuzzer_engine
```

Learn:

- mutation strategies and corpus evolution
- target harness isolation
- feedback signal collection and scheduling
- crash minimization and reproduction flow

Guide:

- [fuzzer_engine_TODO.md](projects/planned/rating_9/fuzzer_engine_TODO.md)

---

## ⭐ 9/10 — Code Sandbox

Goal: Policy-driven isolated execution

Build:

```bash
cargo run --bin code_sandbox
```

Learn:

- declarative sandbox policy modeling
- backend abstraction for isolation mechanisms
- resource accounting and exit classification
- output capture and audit boundaries

Guide:

- [code_sandbox_TODO.md](projects/planned/rating_9/code_sandbox_TODO.md)

---

## ⭐ 8/10 — Artifact Proxy

Goal: Upstream package proxy + integrity cache

Build:

```bash
cargo run --bin artifact_proxy
```

Learn:

- metadata versus blob caching rules
- upstream fetch and proxy semantics
- auth, namespace, and air-gap policy design
- integrity verification for cached artifacts

Guide:

- [artifact_proxy_TODO.md](projects/planned/rating_8/artifact_proxy_TODO.md)

---

## ⭐ 6/10 — RSS Pipeline

Goal: Feed ingestion + transform pipeline

Build:

```bash
cargo run --bin rss_pipeline
```

Learn:

- RSS or Atom fetch and parse stages
- transform-chain ordering and enrichment
- item deduplication across polling runs
- sink abstractions for downstream delivery

Guide:

- [rss_pipeline_TODO.md](projects/planned/rating_6/rss_pipeline_TODO.md)

---

## ⭐ 8/10 — DNS Authority

Goal: Authoritative zone serving + delegation rules

Build:

```bash
cargo run --bin dns_authority
```

Learn:

- zone ownership and record storage design
- authoritative response construction
- SOA, NS, and negative response behavior
- zone reload and delegation boundary handling

Guide:

- [dns_authority_TODO.md](projects/planned/rating_8/dns_authority_TODO.md)

---

## ⭐ 3/10 — Unit Converter

Goal: Simple unit conversions from CLI input

Build:

```bash
cargo run --bin unit_converter
```

Learn:

- CLI parsing for values and units
- small conversion-table design
- category validation for compatible units
- stable numeric formatting

Guide:

- [unit_converter_TODO.md](projects/planned/rating_3/unit_converter_TODO.md)

---

## ⭐ 3/10 — Checksum Tool

Goal: File hashing + checksum verification

Build:

```bash
cargo run --bin checksum_tool
```

Learn:

- file or stdin hashing flow
- digest formatting for human and script use
- manifest verification behavior
- deterministic traversal if extended to directories

Guide:

- [checksum_tool_TODO.md](projects/planned/rating_3/checksum_tool_TODO.md)

---

## ⭐ 4/10 — QR Tool

Goal: QR encode/decode utility

Build:

```bash
cargo run --bin qr_tool
```

Learn:

- text-to-QR generation flow
- terminal or image rendering output
- basic decode-path handling
- round-trip fixture testing

Guide:

- [qr_tool_TODO.md](projects/planned/rating_4/qr_tool_TODO.md)

---

## ⭐ 3/10 — Weather CLI

Goal: Current weather + short forecast output

Build:

```bash
cargo run --bin weather_cli
```

Learn:

- simple CLI argument parsing
- forecast response shaping and formatting
- resilient handling of missing fields
- terminal layout for compact summaries

Guide:

- [weather_cli_TODO.md](projects/planned/rating_3/weather_cli_TODO.md)

---

## ⭐ 4/10 — Batch Renamer

Goal: Safe bulk filename transformation

Build:

```bash
cargo run --bin batch_renamer
```

Learn:

- rename-plan generation before mutation
- transform composition for filenames
- collision detection and dry-run previews
- filesystem mutation safety basics

Guide:

- [batch_renamer_TODO.md](projects/planned/rating_4/batch_renamer_TODO.md)

---

## ⭐ 4/10 — Markdown Reader

Goal: Terminal markdown viewing

Build:

```bash
cargo run --bin markdown_reader
```

Learn:

- markdown parsing versus render separation
- heading/list/code-block display rules
- wrapping and narrow-terminal behavior
- fixture-driven output checks

Guide:

- [markdown_reader_TODO.md](projects/planned/rating_4/markdown_reader_TODO.md)

---

## ⭐ 3/10 — Duplicate Line Filter

Goal: Remove or count repeated lines

Build:

```bash
cargo run --bin duplicate_line_filter
```

Learn:

- streaming line processing basics
- equality versus normalized-text comparison
- keep-first/keep-last style output rules
- simple text-fixture testing

Guide:

- [duplicate_line_filter_TODO.md](projects/planned/rating_3/duplicate_line_filter_TODO.md)

---

## ⭐ 4/10 — TOTP Manager

Goal: Local TOTP code generator

Build:

```bash
cargo run --bin totp_manager
```

Learn:

- shared-secret parsing and validation
- time-step based code generation
- named-account storage layout
- RFC-style vector testing

Guide:

- [totp_manager_TODO.md](projects/planned/rating_4/totp_manager_TODO.md)

---

## ⭐ 2/10 — Calculator CLI

Goal: Evaluate basic arithmetic expressions

Build:

```bash
cargo run --bin calculator_cli
```

Learn:

- expression tokenization basics
- operator precedence handling
- simple evaluation pipelines
- clear malformed-input errors

Guide:

- [calculator_cli_TODO.md](projects/completed/calculator_cli_TODO.md)

---

## ⭐ 3/10 — JSON Viewer

Goal: Pretty-print JSON data

Build:

```bash
cargo run --bin json_viewer
```

Learn:

- JSON input parsing
- compact versus pretty rendering
- parse-error reporting
- stable formatting for fixtures

Guide:

- [json_viewer_TODO.md](projects/planned/rating_3/json_viewer_TODO.md)

---

## ⭐ 2/10 — Timer CLI

Goal: Countdown and stopwatch utility

Build:

```bash
cargo run --bin timer_cli
```

Learn:

- duration parsing from CLI args
- display formatting for elapsed time
- countdown versus stopwatch mode split
- invalid-duration handling

Guide:

- [timer_cli_TODO.md](projects/planned/rating_2/timer_cli_TODO.md)

---

## ⭐ 4/10 — Password Store CLI

Goal: Simple encrypted secret storage

Build:

```bash
cargo run --bin password_store_cli
```

Learn:

- local secret metadata versus value storage
- simple encrypt/decrypt boundaries
- CRUD command design for secrets
- bad-key and round-trip validation

Guide:

- [password_store_cli_TODO.md](projects/planned/rating_4/password_store_cli_TODO.md)

---

## ⭐ 3/10 — CSV Pretty Printer

Goal: Render CSV as aligned tables

Build:

```bash
cargo run --bin csv_pretty_printer
```

Learn:

- CSV parsing basics
- width calculation and aligned terminal output
- quoted-field edge cases
- header versus no-header modes

Guide:

- [csv_pretty_printer_TODO.md](projects/planned/rating_3/csv_pretty_printer_TODO.md)

---

## ⭐ 4/10 — Text Diff CLI

Goal: Line-by-line text comparison

Build:

```bash
cargo run --bin text_diff_cli
```

Learn:

- basic diff algorithm structure
- added/removed/unchanged rendering
- newline and empty-file edge cases
- fixture-based diff output checks

Guide:

- [text_diff_cli_TODO.md](projects/planned/rating_4/text_diff_cli_TODO.md)

---

## ⭐ 3/10 — File Splitter

Goal: Split files by size or lines

Build:

```bash
cargo run --bin file_splitter
```

Learn:

- line-count versus byte-count split logic
- deterministic part naming
- tiny-file and boundary-case handling
- optional reassembly metadata

Guide:

- [file_splitter_TODO.md](projects/planned/rating_3/file_splitter_TODO.md)

---

## ⭐ 3/10 — Env Linter

Goal: Detect common `.env` file issues

Build:

```bash
cargo run --bin env_linter
```

Learn:

- `.env` parsing basics
- duplicate-key and invalid-name rules
- lint rule separation from parsing
- human versus machine-readable diagnostics

Guide:

- [env_linter_TODO.md](projects/planned/rating_3/env_linter_TODO.md)

---

## ⭐ 2/10 — Base64 CLI

Goal: Encode and decode Base64 data

Build:

```bash
cargo run --bin base64_cli
```

Learn:

- Base64 encode/decode flow
- stdin/stdout handling basics
- invalid-input reporting
- round-trip testing

Guide:

- [base64_cli_TODO.md](projects/planned/rating_2/base64_cli_TODO.md)

---

## ⭐ 3/10 — Hex Viewer

Goal: Inspect binary files in hex

Build:

```bash
cargo run --bin hex_viewer
```

Learn:

- binary input reading
- offset and row layout formatting
- ASCII side-column rendering
- stable binary-fixture tests

Guide:

- [hex_viewer_TODO.md](projects/planned/rating_3/hex_viewer_TODO.md)

---

## ⭐ 2/10 — UUID Generator

Goal: Generate UUIDs from CLI

Build:

```bash
cargo run --bin uuid_generator
```

Learn:

- simple random identifier generation
- output formatting choices
- count-based batch output
- format validation tests

Guide:

- [uuid_generator_TODO.md](projects/planned/rating_2/uuid_generator_TODO.md)

---

## ⭐ 2/10 — Gitignore Gen

Goal: Generate `.gitignore` presets

Build:

```bash
cargo run --bin gitignore_gen
```

Learn:

- preset composition for ignore rules
- duplicate removal with stable ordering
- stdout versus file-output behavior
- simple preset data modeling

Guide:

- [gitignore_gen_TODO.md](projects/planned/rating_2/gitignore_gen_TODO.md)

---

## ⭐ 4/10 — HTTP Client CLI

Goal: Send simple HTTP requests

Build:

```bash
cargo run --bin http_client_cli
```

Learn:

- request-building from CLI flags
- response formatting for headers and body
- timeout and error reporting
- helper-level request tests

Guide:

- [http_client_cli_TODO.md](projects/planned/rating_4/http_client_cli_TODO.md)

---

## ⭐ 4/10 — Archive Extractor

Goal: List and extract simple archives

Build:

```bash
cargo run --bin archive_extractor
```

Learn:

- archive-type detection basics
- listing versus extraction flow split
- path-traversal safety checks
- extraction-layout fixtures

Guide:

- [archive_extractor_TODO.md](projects/planned/rating_4/archive_extractor_TODO.md)

---

## ⭐ 3/10 — File Finder

Goal: Recursively search files by pattern

Build:

```bash
cargo run --bin file_finder
```

Learn:

- recursive directory traversal
- pattern-based filtering
- stable result ordering
- traversal-edge-case testing

Guide:

- [file_finder_TODO.md](projects/planned/rating_3/file_finder_TODO.md)

---

## ⭐ 2/10 — Color Preview

Goal: Preview colors in the terminal

Build:

```bash
cargo run --bin color_preview
```

Learn:

- hex or RGB parsing
- terminal color swatch rendering
- multi-color preview formatting
- malformed-color validation

Guide:

- [color_preview_TODO.md](projects/planned/rating_2/color_preview_TODO.md)

---

## ⭐ 2/10 — WC Clone

Goal: Count lines, words, and bytes

Build:

```bash
cargo run --bin wc_clone
```

Learn:

- streaming text input basics
- line, word, and byte counting logic
- output formatting similar to small Unix tools
- fixture-driven counting tests

Guide:

- [wc_clone_TODO.md](projects/planned/rating_2/wc_clone_TODO.md)

---

## ⭐ 3/10 — Line Ending Converter

Goal: Convert text between LF and CRLF

Build:

```bash
cargo run --bin line_ending_converter
```

Learn:

- newline normalization strategies
- stdout versus in-place file updates
- preserving trailing-newline behavior
- mixed-line-ending fixture tests

Guide:

- [line_ending_converter_TODO.md](projects/planned/rating_3/line_ending_converter_TODO.md)

---

## ⭐ 2/10 — Slug Generator

Goal: Turn text into URL-friendly slugs

Build:

```bash
cargo run --bin slug_generator
```

Learn:

- lowercase normalization
- separator collapsing rules
- punctuation cleanup
- transformation-step unit tests

Guide:

- [slug_generator_TODO.md](projects/planned/rating_2/slug_generator_TODO.md)

---

## ⭐ 3/10 — Notes CLI

Goal: Store and manage simple notes

Build:

```bash
cargo run --bin notes_cli
```

Learn:

- small local persistence design
- add/list/delete command structure
- simple record identifiers
- storage round-trip tests

Guide:

- [notes_cli_TODO.md](projects/planned/rating_3/notes_cli_TODO.md)

---

## ⭐ 2/10 — ASCII Table

Goal: Print an ASCII reference table

Build:

```bash
cargo run --bin ascii_table
```

Learn:

- tabular CLI output formatting
- control-character labeling
- numeric base conversions
- snapshot-like rendering tests

Guide:

- [ascii_table_TODO.md](projects/planned/rating_2/ascii_table_TODO.md)

---

## ⭐ 2/10 — URL Codec

Goal: Encode and decode URL components

Build:

```bash
cargo run --bin url_codec
```

Learn:

- percent-encoding rules
- malformed escape validation
- mode-based CLI behavior
- symbol-heavy input tests

Guide:

- [url_codec_TODO.md](projects/planned/rating_2/url_codec_TODO.md)

---

## ⭐ 4/10 — INI Inspector

Goal: Inspect and query INI files

Build:

```bash
cargo run --bin ini_inspector
```

Learn:

- section and key/value parsing
- duplicate-key diagnostics
- query-oriented data modeling
- fixture-based parser tests

Guide:

- [ini_inspector_TODO.md](projects/planned/rating_4/ini_inspector_TODO.md)

---

## ⭐ 4/10 — Date Calculator

Goal: Perform simple date arithmetic

Build:

```bash
cargo run --bin date_calculator
```

Learn:

- date parsing and formatting
- day-based addition and subtraction
- date-difference calculations
- boundary tests for leap years and month rollover

Guide:

- [date_calculator_TODO.md](projects/planned/rating_4/date_calculator_TODO.md)

---

## ⭐ 6/10 — Feature Flag Service

Goal: Evaluate feature flags over an API

Build:

```bash
cargo run --bin feature_flag_service
```

Learn:

- rule-based flag evaluation
- percentage rollout strategies
- environment and targeting models
- deterministic evaluation tests

Guide:

- [feature_flag_service_TODO.md](projects/planned/rating_6/feature_flag_service_TODO.md)

---

## ⭐ 5/10 — Cron Scheduler

Goal: Run jobs on cron-like schedules

Build:

```bash
cargo run --bin cron_scheduler
```

Learn:

- cron-expression parsing basics
- next-run time calculation
- graceful shutdown for long-lived workers
- schedule boundary testing

Guide:

- [cron_scheduler_TODO.md](projects/planned/rating_5/cron_scheduler_TODO.md)

---

## ⭐ 5/10 — API Mock Server

Goal: Serve configurable mock API responses

Build:

```bash
cargo run --bin api_mock_server
```

Learn:

- config-driven route definitions
- method and path matching
- fixture-backed response rendering
- response-behavior tests

Guide:

- [api_mock_server_TODO.md](projects/planned/rating_5/api_mock_server_TODO.md)

---

## ⭐ 6/10 — Backup Tool

Goal: Create and restore filesystem snapshots

Build:

```bash
cargo run --bin backup_tool
```

Learn:

- snapshot layout design
- unchanged-file detection
- restore-path safety
- exclusion and manifest testing

Guide:

- [backup_tool_TODO.md](projects/planned/rating_6/backup_tool_TODO.md)

---

## ⭐ 7/10 — Notebook Sync Engine

Goal: Sync notebooks and detect conflicts

Build:

```bash
cargo run --bin notebook_sync_engine
```

Learn:

- change-set modeling
- conflict detection between divergent edits
- merge-marker or reconciliation strategies
- sync-checkpoint testing

Guide:

- [notebook_sync_engine_TODO.md](projects/planned/rating_7/notebook_sync_engine_TODO.md)

---

## ⭐ 7/10 — Search Indexer

Goal: Build and query an inverted search index

Build:

```bash
cargo run --bin search_indexer
```

Learn:

- tokenization and document crawling
- inverted-index data structures
- simple query parsing and ranking
- indexing and retrieval tests

Guide:

- [search_indexer_TODO.md](projects/planned/rating_7/search_indexer_TODO.md)

---

## ⭐ 7/10 — Media Library Server

Goal: Catalog and browse a media library

Build:

```bash
cargo run --bin media_library_server
```

Learn:

- metadata extraction flow
- normalized catalog design
- browse and search endpoints
- scan-result fixture testing

Guide:

- [media_library_server_TODO.md](projects/planned/rating_7/media_library_server_TODO.md)

---

## ⭐ 8/10 — Package Registry

Goal: Publish and fetch package artifacts

Build:

```bash
cargo run --bin package_registry
```

Learn:

- package/version metadata modeling
- immutable artifact rules
- upload and download API design
- conflict and authorization testing

Guide:

- [package_registry_TODO.md](projects/planned/rating_8/package_registry_TODO.md)

---

## ⭐ 7/10 — CI Runner

Goal: Execute CI jobs with isolated steps

Build:

```bash
cargo run --bin ci_runner
```

Learn:

- job and step modeling
- workspace and artifact lifecycle design
- failure propagation and retries
- deterministic test harnesses for pipelines

Guide:

- [ci_runner_TODO.md](projects/planned/rating_7/ci_runner_TODO.md)

---

## ⭐ 6/10 — Metrics Dashboard

Goal: Aggregate and visualize runtime metrics

Build:

```bash
cargo run --bin metrics_dashboard
```

Learn:

- timeseries aggregation basics
- panel and query abstractions
- threshold and alert primitives
- fixture-driven dashboard tests

Guide:

- [metrics_dashboard_TODO.md](projects/planned/rating_6/metrics_dashboard_TODO.md)

---

## ⭐ 8/10 — Plugin Runtime

Goal: Load and run sandboxed plugins

Build:

```bash
cargo run --bin plugin_runtime
```

Learn:

- plugin ABI boundary modeling
- capability-based isolation patterns
- lifecycle hooks and resource controls
- integration tests for host-plugin contracts

Guide:

- [plugin_runtime_TODO.md](projects/planned/rating_8/plugin_runtime_TODO.md)

---

## ⭐ 5/10 — Notification Hub

Goal: Route notifications to multiple sinks

Build:

```bash
cargo run --bin notification_hub
```

Learn:

- event envelope design
- routing by severity or topic
- sink abstraction and retries
- tests for delivery behavior

Guide:

- [notification_hub_TODO.md](projects/planned/rating_5/notification_hub_TODO.md)

---

## ⭐ 6/10 — Config Deployer

Goal: Validate and roll out configuration changes

Build:

```bash
cargo run --bin config_deployer
```

Learn:

- config schema and validation flow
- staged rollout with rollback hooks
- drift detection and reconciliation basics
- tests for safe deployment gates

Guide:

- [config_deployer_TODO.md](projects/planned/rating_6/config_deployer_TODO.md)

---

## ⭐ 5/10 — Artifact Signer

Goal: Sign and verify build artifacts

Build:

```bash
cargo run --bin artifact_signer
```

Learn:

- digest and signature pipeline design
- key abstraction boundaries
- verification and trust policy checks
- tests for tamper and mismatch cases

Guide:

- [artifact_signer_TODO.md](projects/planned/rating_5/artifact_signer_TODO.md)

---

## ⭐ 7/10 — Job Queue Server

Goal: Manage durable queues and worker leases

Build:

```bash
cargo run --bin job_queue_server
```

Learn:

- durable queue semantics
- worker lease and visibility timeout logic
- retry and dead-letter handling
- concurrency tests for claim/ack paths

Guide:

- [job_queue_server_TODO.md](projects/planned/rating_7/job_queue_server_TODO.md)

---

## ⭐ 6/10 — Webhook Dispatcher

Goal: Deliver webhooks with retry/backoff

Build:

```bash
cargo run --bin webhook_dispatcher
```

Learn:

- delivery contract and idempotency keys
- exponential backoff policy design
- payload signing and endpoint auth basics
- tests for retry exhaustion behavior

Guide:

- [webhook_dispatcher_TODO.md](projects/planned/rating_6/webhook_dispatcher_TODO.md)

---

## ⭐ 6/10 — Incident Router

Goal: Route incidents with escalation policies

Build:

```bash
cargo run --bin incident_router
```

Learn:

- incident classification and envelope design
- rule-based team routing
- escalation and fallback semantics
- deterministic tests for noisy-event handling

Guide:

- [incident_router_TODO.md](projects/planned/rating_6/incident_router_TODO.md)

---

## ⭐ 7/10 — SBOM Scanner

Goal: Build SBOM reports and risk summaries

Build:

```bash
cargo run --bin sbom_scanner
```

Learn:

- dependency graph extraction
- normalized package identity modeling
- report generation and severity scoring
- fixture-driven vulnerability matching tests

Guide:

- [sbom_scanner_TODO.md](projects/planned/rating_7/sbom_scanner_TODO.md)

---

## ⭐ 6/10 — Edge Cache

Goal: Cache and revalidate edge responses

Build:

```bash
cargo run --bin edge_cache
```

Learn:

- cache key normalization
- TTL and eviction policies
- conditional revalidation behavior
- tests for freshness and stale paths

Guide:

- [edge_cache_TODO.md](projects/planned/rating_6/edge_cache_TODO.md)

---

## ⭐ 8/10 — Canary Controller

Goal: Automate canary promotion and rollback

Build:

```bash
cargo run --bin canary_controller
```

Learn:

- rollout stage state machines
- health-signal scoring and thresholds
- promote/rollback transition safety
- tests for failure-driven rollback paths

Guide:

- [canary_controller_TODO.md](projects/planned/rating_8/canary_controller_TODO.md)

---

## ⭐ 5/10 — Quota Service

Goal: Enforce tenant and endpoint quotas

Build:

```bash
cargo run --bin quota_service
```

Learn:

- quota dimension modeling
- windowing algorithms
- deterministic clock abstractions
- tests around limit boundaries

Guide:

- [quota_service_TODO.md](projects/planned/rating_5/quota_service_TODO.md)

---

## ⭐ 6/10 — Audit Trail Store

Goal: Append-only audit event storage

Build:

```bash
cargo run --bin audit_trail_store
```

Learn:

- immutable event log design
- event ordering and filtering indexes
- hash-chain tamper evidence basics
- tests for integrity and query correctness

Guide:

- [audit_trail_store_TODO.md](projects/planned/rating_6/audit_trail_store_TODO.md)

---

## ⭐ 7/10 — Rollout Manager

Goal: Execute staged feature rollouts safely

Build:

```bash
cargo run --bin rollout_manager
```

Learn:

- rollout plan representation
- checkpointing and resumability
- blast-radius and pause controls
- tests for progression and cancellation

Guide:

- [rollout_manager_TODO.md](projects/planned/rating_7/rollout_manager_TODO.md)

---

## ⭐ 7/10 — Dependency Mirror

Goal: Mirror dependency metadata and artifacts

Build:

```bash
cargo run --bin dependency_mirror
```

Learn:

- metadata/artifact consistency modeling
- digest verification and trust boundaries
- cache freshness and fallback strategy
- tests for mirror failover behavior

Guide:

- [dependency_mirror_TODO.md](projects/planned/rating_7/dependency_mirror_TODO.md)

---

## ⭐ 8/10 — Chaos Orchestrator

Goal: Run controlled fault injection experiments

Build:

```bash
cargo run --bin chaos_orchestrator
```

Learn:

- experiment and blast-radius modeling
- fault injection adapters
- guardrails and stop conditions
- tests for safety and rollback behavior

Guide:

- [chaos_orchestrator_TODO.md](projects/planned/rating_8/chaos_orchestrator_TODO.md)

---

## ⭐ 5/10 — Session Store

Goal: Manage expiring user sessions safely

Build:

```bash
cargo run --bin session_store
```

Learn:

- session lifecycle modeling
- token identity and TTL semantics
- refresh and invalidation patterns
- tests for expiration edge cases

Guide:

- [session_store_TODO.md](projects/planned/rating_5/session_store_TODO.md)

---

## ⭐ 6/10 — API Contract Tester

Goal: Validate APIs against contract expectations

Build:

```bash
cargo run --bin api_contract_tester
```

Learn:

- contract matcher design
- request execution abstraction
- schema and status assertions
- failure diff reporting tests

Guide:

- [api_contract_tester_TODO.md](projects/planned/rating_6/api_contract_tester_TODO.md)

---

## ⭐ 6/10 — Cache Invalidator

Goal: Coordinate distributed cache invalidation

Build:

```bash
cargo run --bin cache_invalidator
```

Learn:

- invalidation event modeling
- fan-out and retry policies
- idempotency handling
- tests for propagation correctness

Guide:

- [cache_invalidator_TODO.md](projects/planned/rating_6/cache_invalidator_TODO.md)

---

## ⭐ 7/10 — Compliance Checker

Goal: Evaluate resources against policy controls

Build:

```bash
cargo run --bin compliance_checker
```

Learn:

- policy rule evaluation
- evidence collection and scoring
- waiver lifecycle handling
- tests for report integrity

Guide:

- [compliance_checker_TODO.md](projects/planned/rating_7/compliance_checker_TODO.md)

---

## ⭐ 5/10 — Synthetic Probe

Goal: Run scheduled probes and health checks

Build:

```bash
cargo run --bin synthetic_probe
```

Learn:

- probe scenario scheduling
- timeout and retry strategy
- latency/result metric collection
- tests for timing behavior

Guide:

- [synthetic_probe_TODO.md](projects/planned/rating_5/synthetic_probe_TODO.md)

---

## ⭐ 8/10 — Threat Feed Aggregator

Goal: Aggregate and score threat intelligence feeds

Build:

```bash
cargo run --bin threat_feed_aggregator
```

Learn:

- multi-source indicator ingestion
- canonical normalization and dedup
- source weighting and confidence scoring
- tests for merge and provenance behavior

Guide:

- [threat_feed_aggregator_TODO.md](projects/planned/rating_8/threat_feed_aggregator_TODO.md)

---

## ⭐ 7/10 — Tenant Provisioner

Goal: Provision tenant resources with rollback safety

Build:

```bash
cargo run --bin tenant_provisioner
```

Learn:

- workflow checkpointing
- idempotent provisioning steps
- rollback on partial failures
- tests for retry and recovery logic

Guide:

- [tenant_provisioner_TODO.md](projects/planned/rating_7/tenant_provisioner_TODO.md)

---

## ⭐ 6/10 — Release Train

Goal: Coordinate gated release train promotions

Build:

```bash
cargo run --bin release_train
```

Learn:

- release window and cutoff modeling
- promotion gate orchestration
- checkpointed rollout and rollback flow
- tests for gate failure handling

Guide:

- [release_train_TODO.md](projects/planned/rating_6/release_train_TODO.md)

---

## ⭐ 5/10 — Incident Timeline

Goal: Build incident timelines from event streams

Build:

```bash
cargo run --bin incident_timeline
```

Learn:

- event normalization and ordering
- merge rules across event sources
- timeline rendering strategies
- tests for ordering and dedup behavior

Guide:

- [incident_timeline_TODO.md](projects/planned/rating_5/incident_timeline_TODO.md)

---

## ⭐ 6/10 — Policy Simulator

Goal: Dry-run policy changes with explain traces

Build:

```bash
cargo run --bin policy_simulator
```

Learn:

- policy parse and evaluation split
- dry-run decision paths
- explain trace construction
- tests for rule matching correctness

Guide:

- [policy_simulator_TODO.md](projects/planned/rating_6/policy_simulator_TODO.md)

---

## ⭐ 7/10 — Dependency Auditor

Goal: Audit dependency graphs against policy

Build:

```bash
cargo run --bin dependency_auditor
```

Learn:

- graph extraction from manifests
- vulnerability/license/trust checks
- report shaping for remediation
- tests for traversal and policy matching

Guide:

- [dependency_auditor_TODO.md](projects/planned/rating_7/dependency_auditor_TODO.md)

---

## ⭐ 7/10 — Traffic Replayer

Goal: Replay production-like request traffic safely

Build:

```bash
cargo run --bin traffic_replayer
```

Learn:

- capture format and schedule fidelity
- concurrency and pacing controls
- endpoint mapping and mutation hooks
- tests for deterministic replay behavior

Guide:

- [traffic_replayer_TODO.md](projects/planned/rating_7/traffic_replayer_TODO.md)

---

## ⭐ 8/10 — Failover Coordinator

Goal: Orchestrate failover and safe failback decisions

Build:

```bash
cargo run --bin failover_coordinator
```

Learn:

- topology and health policy modeling
- decision hysteresis to avoid flapping
- persisted transition history
- tests for split-brain scenarios

Guide:

- [failover_coordinator_TODO.md](projects/planned/rating_8/failover_coordinator_TODO.md)

---

## ⭐ 6/10 — Tenant Billing Meter

Goal: Meter tenant usage for billing pipelines

Build:

```bash
cargo run --bin tenant_billing_meter
```

Learn:

- usage event schema and aggregation
- pricing tier rule application
- invoice-ready summary modeling
- tests for threshold and rounding behavior

Guide:

- [tenant_billing_meter_TODO.md](projects/planned/rating_6/tenant_billing_meter_TODO.md)

---

## ⭐ 5/10 — Runtime Profiler

Goal: Summarize runtime hotspots and regressions

Build:

```bash
cargo run --bin runtime_profiler
```

Learn:

- sampling and span attribution
- hotspot ranking and percentile math
- regression threshold alerts
- tests for aggregate metric correctness

Guide:

- [runtime_profiler_TODO.md](projects/planned/rating_5/runtime_profiler_TODO.md)

---

## ⭐ 6/10 — Service Catalog

Goal: Maintain service metadata and dependencies

Build:

```bash
cargo run --bin service_catalog
```

Learn:

- service identity and ownership modeling
- dependency graph representation
- query and search index tradeoffs
- tests for relationship integrity

Guide:

- [service_catalog_TODO.md](projects/planned/rating_6/service_catalog_TODO.md)

---

## ⭐ 5/10 — Incident Postmortem

Goal: Generate postmortems from incident records

Build:

```bash
cargo run --bin incident_postmortem
```

Learn:

- postmortem schema and template rendering
- timeline and factor ingestion flow
- action-item ownership modeling
- tests for required section completeness

Guide:

- [incident_postmortem_TODO.md](projects/planned/rating_5/incident_postmortem_TODO.md)

---

## ⭐ 7/10 — Access Review Engine

Goal: Run periodic entitlement access reviews

Build:

```bash
cargo run --bin access_review_engine
```

Learn:

- entitlement graph modeling
- review campaign lifecycle
- decision evidence and escalation flow
- tests for revoke/approve rules

Guide:

- [access_review_engine_TODO.md](projects/planned/rating_7/access_review_engine_TODO.md)

---

## ⭐ 7/10 — Secret Rotation Service

Goal: Coordinate scheduled secret rotations safely

Build:

```bash
cargo run --bin secret_rotation_service
```

Learn:

- versioned secret lifecycle
- rotation scheduling and rollout orchestration
- rollback and grace-period handling
- tests for version promotion safety

Guide:

- [secret_rotation_service_TODO.md](projects/planned/rating_7/secret_rotation_service_TODO.md)

---

## ⭐ 6/10 — Capacity Planner

Goal: Forecast capacity and recommend scaling actions

Build:

```bash
cargo run --bin capacity_planner
```

Learn:

- workload signal aggregation
- forecast and saturation heuristics
- scenario analysis modeling
- tests for threshold behavior

Guide:

- [capacity_planner_TODO.md](projects/planned/rating_6/capacity_planner_TODO.md)

---

## ⭐ 5/10 — Uptime SLA Tracker

Goal: Measure SLA compliance and error budgets

Build:

```bash
cargo run --bin uptime_sla_tracker
```

Learn:

- objective and window definitions
- downtime interval accounting
- breach and burn-rate signals
- tests for boundary calculations

Guide:

- [uptime_sla_tracker_TODO.md](projects/planned/rating_5/uptime_sla_tracker_TODO.md)

---

## ⭐ 6/10 — Config Diff Auditor

Goal: Audit semantic configuration changes

Build:

```bash
cargo run --bin config_diff_auditor
```

Learn:

- typed config parsing
- semantic diff and ignore rules
- risk classification policies
- tests for nested diff behavior

Guide:

- [config_diff_auditor_TODO.md](projects/planned/rating_6/config_diff_auditor_TODO.md)

---

## ⭐ 7/10 — API Replay Harness

Goal: Replay API sessions and compare baselines

Build:

```bash
cargo run --bin api_replay_harness
```

Learn:

- session capture and ordering constraints
- deterministic replay scheduling
- response diff and masking logic
- tests for stable comparison output

Guide:

- [api_replay_harness_TODO.md](projects/planned/rating_7/api_replay_harness_TODO.md)

---

## ⭐ 7/10 — Change Approval Engine

Goal: Evaluate and route change approvals

Build:

```bash
cargo run --bin change_approval_engine
```

Learn:

- risk-aware approval modeling
- reviewer quorum and escalation flow
- immutable decision trails
- tests for timeout and escalation logic

Guide:

- [change_approval_engine_TODO.md](projects/planned/rating_7/change_approval_engine_TODO.md)

---

## ⭐ 6/10 — Log Redaction Gateway

Goal: Redact sensitive fields from log streams

Build:

```bash
cargo run --bin log_redaction_gateway
```

Learn:

- redaction rule pipelines
- deterministic masking strategies
- context preservation for observability
- tests for redaction safety

Guide:

- [log_redaction_gateway_TODO.md](projects/planned/rating_6/log_redaction_gateway_TODO.md)

---

## ⭐ 8/10 — Schema Migration Planner

Goal: Plan safe schema migrations with rollbacks

Build:

```bash
cargo run --bin schema_migration_planner
```

Learn:

- versioned schema evolution modeling
- compatibility analysis gates
- ordered migration planning
- tests for dependency and rollback behavior

Guide:

- [schema_migration_planner_TODO.md](projects/planned/rating_8/schema_migration_planner_TODO.md)

---

## ⭐ 6/10 — Incident Simulator

Goal: Simulate incident scenarios and response timings

Build:

```bash
cargo run --bin incident_simulator
```

Learn:

- scenario template modeling
- seeded simulation reproducibility
- detection/mitigation latency metrics
- tests for deterministic execution

Guide:

- [incident_simulator_TODO.md](projects/planned/rating_6/incident_simulator_TODO.md)

---

## ⭐ 5/10 — API Deprecation Tracker

Goal: Track API sunset timelines and usage risk

Build:

```bash
cargo run --bin api_deprecation_tracker
```

Learn:

- deprecation window modeling
- usage ingestion and risk scoring
- deadline notification mechanics
- tests for date boundary logic

Guide:

- [api_deprecation_tracker_TODO.md](projects/planned/rating_5/api_deprecation_tracker_TODO.md)

---

## ⭐ 6/10 — Workload Forecaster

Goal: Forecast workload demand and variance

Build:

```bash
cargo run --bin workload_forecaster
```

Learn:

- trend/seasonality forecast basics
- confidence interval estimation
- forecast export contracts
- tests for anomaly resilience

Guide:

- [workload_forecaster_TODO.md](projects/planned/rating_6/workload_forecaster_TODO.md)

---

## ⭐ 6/10 — Credential Inventory

Goal: Inventory credentials and detect stale access

Build:

```bash
cargo run --bin credential_inventory
```

Learn:

- metadata-only credential cataloging
- stale/orphan detection rules
- remediation queue generation
- tests for ownership logic

Guide:

- [credential_inventory_TODO.md](projects/planned/rating_6/credential_inventory_TODO.md)

---

## ⭐ 7/10 — Delivery SLO Guard

Goal: Guard delivery SLOs with burn-rate signals

Build:

```bash
cargo run --bin delivery_slo_guard
```

Learn:

- windowed SLO compliance calculations
- burn-rate thresholding
- mitigation recommendation flows
- tests for time-window math

Guide:

- [delivery_slo_guard_TODO.md](projects/planned/rating_7/delivery_slo_guard_TODO.md)

---

## ⭐ 7/10 — Blast Radius Analyzer

Goal: Estimate impact scope from dependency graphs

Build:

```bash
cargo run --bin blast_radius_analyzer
```

Learn:

- dependency graph normalization
- impact traversal and scoring
- confidence estimation for partial data
- tests for traversal determinism

Guide:

- [blast_radius_analyzer_TODO.md](projects/planned/rating_7/blast_radius_analyzer_TODO.md)

---

## ⭐ 6/10 — Runbook Recommender

Goal: Recommend operational runbooks for incidents

Build:

```bash
cargo run --bin runbook_recommender
```

Learn:

- runbook retrieval strategies
- ranking with feedback loops
- failure-mode to runbook mapping
- tests for ranking consistency

Guide:

- [runbook_recommender_TODO.md](projects/planned/rating_6/runbook_recommender_TODO.md)

---

## ⭐ 5/10 — Maintenance Window Manager

Goal: Plan and validate maintenance windows

Build:

```bash
cargo run --bin maintenance_window_manager
```

Learn:

- recurrence and overlap rules
- timezone-safe schedule math
- policy checks for blackout periods
- tests for boundary conditions

Guide:

- [maintenance_window_manager_TODO.md](projects/planned/rating_5/maintenance_window_manager_TODO.md)

---

## ⭐ 6/10 — Latency Budget Planner

Goal: Allocate and track end-to-end latency budgets

Build:

```bash
cargo run --bin latency_budget_planner
```

Learn:

- per-hop budget allocation
- percentile-aware budget accounting
- bottleneck detection heuristics
- tests for budget propagation logic

Guide:

- [latency_budget_planner_TODO.md](projects/planned/rating_6/latency_budget_planner_TODO.md)

---

## ⭐ 7/10 — Release Guardrail

Goal: Enforce release readiness guard checks

Build:

```bash
cargo run --bin release_guardrail
```

Learn:

- composable release gates
- health/error/saturation check design
- pause and block decision semantics
- tests for gate ordering and failure behavior

Guide:

- [release_guardrail_TODO.md](projects/planned/rating_7/release_guardrail_TODO.md)

---

## ⭐ 6/10 — Dependency Risk Heatmap

Goal: Visualize dependency risk concentration

Build:

```bash
cargo run --bin dependency_risk_heatmap
```

Learn:

- graph centrality with risk scoring
- ownership-based grouping views
- heatmap output shaping
- tests for scoring stability

Guide:

- [dependency_risk_heatmap_TODO.md](projects/planned/rating_6/dependency_risk_heatmap_TODO.md)

---

## ⭐ 8/10 — Authz Drift Detector

Goal: Detect authorization drift from desired state

Build:

```bash
cargo run --bin authz_drift_detector
```

Learn:

- desired vs observed policy graph diffing
- severity classification by resource sensitivity
- remediation planning workflow
- tests for deterministic drift detection

Guide:

- [authz_drift_detector_TODO.md](projects/planned/rating_8/authz_drift_detector_TODO.md)

---

## ⭐ 6/10 — Incident Comms Broker

Goal: Broker incident updates across channels

Build:

```bash
cargo run --bin incident_comms_broker
```

Learn:

- channel routing and subscriber policies
- ordered fan-out delivery
- idempotency and retry handling
- tests for dedup and delivery semantics

Guide:

- [incident_comms_broker_TODO.md](projects/planned/rating_6/incident_comms_broker_TODO.md)

---

## ⭐ 1/10 — Line Sorter

Goal: Sort and filter file lines

Build:

```bash
cargo run --bin sorter -- file.txt
```

Learn:

- file I/O and line reading
- vector sorting and filtering
- CLI flag parsing
- basic tests

Guide:

- [line_sorter_TODO.md](projects/wip/line_sorter_TODO.md)

---

## ⭐ 1/10 — File Concatenator

Goal: Combine multiple files

Build:

```bash
cargo run --bin concat -- file1.txt file2.txt
```

Learn:

- multiple file handling
- argument parsing
- error handling for missing files
- output redirection

Guide:

- [file_concatenator_TODO.md](projects/wip/file_concatenator_TODO.md)

---

## ⭐ 2/10 — String Case Converter

Goal: Convert between naming conventions

Build:

```bash
cargo run --bin case_convert -- "hello world" to-snake
```

Learn:

- string manipulation and iteration
- character classification
- case conversion logic
- edge case handling

Guide:

- [string_case_converter_TODO.md](projects/wip/string_case_converter_TODO.md)

---

## ⭐ 2/10 — Word Frequency Counter

Goal: Count word occurrences in text

Build:

```bash
cargo run --bin wordfreq -- file.txt
```

Learn:

- HashMap usage for aggregation
- file reading and parsing
- sorting and display
- text normalization

Guide:

- [word_frequency_counter_TODO.md](projects/wip/word_frequency_counter_TODO.md)

---

## ⭐ 2/10 — Text Statistics

Goal: Count lines, words, characters

Build:

```bash
cargo run --bin txtstat -- file.txt
```

Learn:

- counting and aggregation
- CLI flags and options
- formatted output
- edge cases (empty files, Unicode)

Guide:

- [text_statistics_TODO.md](projects/wip/text_statistics_TODO.md)

---

## ⭐ 2/10 — Simple Grep

Goal: Basic text search tool

Build:

```bash
cargo run --bin grepish -- "pattern" file.txt
```

Learn:

- pattern matching and searching
- line-by-line processing
- case sensitivity options
- output formatting

Guide:

- [simple_grep_TODO.md](projects/wip/simple_grep_TODO.md)

---

## ⭐ 2/10 — Number Guessing Game

Goal: Interactive guessing game

Build:

```bash
cargo run --bin number_guess
```

Learn:

- random number generation
- stdin reading and parsing
- game loop implementation
- feedback and scoring

Guide:

- [number_guessing_game_TODO.md](projects/wip/number_guessing_game_TODO.md)

---

## ⭐ 3/10 — Simple Todo App

Goal: CLI task manager with persistence

Build:

```bash
cargo run --bin todocli -- add "Task"
```

Learn:

- struct definition and modeling
- JSON serialization/deserialization
- file I/O for persistence
- CRUD operations
- command parsing

Guide:

- [simple_todo_app_TODO.md](projects/wip/simple_todo_app_TODO.md)
