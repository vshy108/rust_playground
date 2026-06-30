# TODO: web_crawler (⭐ 5/10)

## Usage

```bash
cargo run --bin crawler -- https://example.com
cargo test --bin crawler
```

## 1. Fetch a single page

- [ ] Use `reqwest` to GET a URL; return the response body as a String.
- [ ] Handle non-200 status codes as errors.

Acceptance check: fetching `https://example.com` prints the HTML length.

## 2. Extract links

- [ ] Parse the HTML body; extract all `href` values from `<a>` tags.
- [ ] Resolve relative URLs against the base URL.
- [ ] Filter to same-domain URLs only.

Acceptance check: links extracted from a known HTML string match expected list.

## 3. BFS crawl loop

- [ ] Keep a `visited: HashSet<Url>` to avoid revisiting.
- [ ] Use a `VecDeque` as a frontier queue.
- [ ] Crawl breadth-first; print each discovered URL.

Acceptance check: crawling a local HTML fixture discovers all linked pages without duplicates.

## 4. Async concurrent crawl

- [ ] Spawn a `tokio::task` per URL instead of fetching sequentially.
- [ ] Collect results via `JoinHandle`; feed discovered links back into the frontier.

Acceptance check: crawling runs noticeably faster than the sequential version.

## 5. Tests

- [ ] `extract_links` returns correct URLs from a fixed HTML snippet.
- [ ] Relative URLs are resolved to absolute.
- [ ] Off-domain URLs are filtered out.

## Extra: concurrency limit

- [ ] Add `--concurrency N` flag; use `tokio::sync::Semaphore` to cap in-flight fetches.

## Tips

- Start with a deterministic local fixture path before external integration.
- Add bounded concurrency controls early to prevent overload and flakiness.
- Separate collection from aggregation/output so each can be tested in isolation.
- Add backoff and retry policy tests for transient failures.
- Measure throughput and tail latency on representative input sizes.
