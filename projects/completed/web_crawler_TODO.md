# TODO: web_crawler (⭐ 5/10)


## Usage

```bash
cargo run --bin crawler -- https://example.com
cargo test --bin crawler
```

## 1. Fetch a single page

- [x] Use `reqwest` to GET a URL; return the response body as a String.
- [x] Handle non-200 status codes as errors.

Acceptance check: fetching `https://example.com` prints the HTML length.

## 2. Extract links

- [x] Parse the HTML body; extract all `href` values from `<a>` tags.
- [x] Resolve relative URLs against the base URL.
- [x] Filter to same-domain URLs only.

Acceptance check: links extracted from a known HTML string match expected list.

## 3. BFS crawl loop

- [x] Keep a `visited: HashSet<Url>` to avoid revisiting.
- [x] Use a `VecDeque` as a frontier queue.
- [x] Crawl breadth-first; print each discovered URL.

Acceptance check: crawling a local HTML fixture discovers all linked pages without duplicates.

## 4. Async concurrent crawl

- [x] Spawn a `tokio::task` per URL instead of fetching sequentially.
- [x] Collect results via `JoinHandle`; feed discovered links back into the frontier.

Acceptance check: crawling runs noticeably faster than the sequential version.

## 5. Tests

- [x] `extract_links` returns correct URLs from a fixed HTML snippet.
- [x] Relative URLs are resolved to absolute.
- [x] Off-domain URLs are filtered out.

## Extra: concurrency limit

- [x] Add `--concurrency N` flag; use `tokio::sync::Semaphore` to cap in-flight fetches.

## Status

Completed.

## Specification

- Goal: crawl same-domain links breadth-first with bounded async concurrency.
- Inputs: starting URL, optional depth, and concurrency limit.
- Output: each visited URL once, in breadth-first discovery order.
- Errors: reject invalid URLs/options, non-success responses, malformed links, and failed task joins.
- Non-goals: JavaScript rendering, robots policy, persistence, and cross-domain crawling.
- Acceptance: relative-link, same-domain filtering, empty-page, async crawl, and strict Clippy checks pass.

## Change record

- Implemented reqwest fetching, URL resolution, same-domain filtering, visited tracking, breadth-first frontier management, task concurrency, and semaphore limits.

## Tips

- Start with a deterministic local fixture path before external integration.
- Add bounded concurrency controls early to prevent overload and flakiness.
- Separate collection from aggregation/output so each can be tested in isolation.
- Add backoff and retry policy tests for transient failures.
- Measure throughput and tail latency on representative input sizes.

## Learn Notes

- futures — a Future is a lazy computation; it does nothing until polled by an executor
- task scheduling — `tokio::spawn` creates an independent task; tasks run concurrently on the tokio thread pool; `JoinHandle` lets the spawner await the result
- async channels — `tokio::sync::mpsc` passes URLs between the discovery task and workers without blocking; the channel decouples producers from consumers

## Extra

- limit concurrency with a semaphore (`tokio::sync::Semaphore`)
