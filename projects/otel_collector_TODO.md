# TODO: otel_collector

## Usage

```bash
cargo run --bin otel_collector
cargo test --bin otel_collector
```

## 1. Span model

- [ ] Define a `Span` struct: `trace_id: String`, `span_id: String`, `name: String`,
  `start_ns: u64`, `end_ns: u64`, `attributes: HashMap<String, String>`.

Acceptance check: `Span` serialises/deserialises with serde_json.

## 2. HTTP receiver

- [ ] Accept `POST /v1/traces` with a JSON body containing a list of spans.
- [ ] Deserialise and forward each span into an internal channel.

Acceptance check: posting a JSON array of spans logs "received N spans".

## 3. Batch buffer

- [ ] Buffer incoming spans in a `Vec<Span>` inside an aggregator task.
- [ ] Flush when buffer reaches 100 spans OR 5 seconds since last flush, whichever first.

Acceptance check: 50 spans + 5-second wait triggers a flush of 50.

## 4. Exporter

- [ ] On flush, write spans to stdout as newline-delimited JSON (NDJSON).
- [ ] Track total flushed span count.

Acceptance check: output file contains one JSON object per span.

## 5. Tests

- [ ] Batch flushes at size threshold.
- [ ] Batch flushes at time threshold even when below size.
- [ ] Exporter output is valid NDJSON.

## Extra: metrics + traces

- [ ] Add `POST /v1/metrics` endpoint; route metric events through a separate pipeline.
- [ ] Emit a summary line (spans/metrics per minute) every 60 seconds.
