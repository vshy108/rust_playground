# TODO: http_client_cli (⭐ 4/10)

## Usage

```bash
cargo run --bin http_client_cli
cargo test --bin http_client_cli
```

## Milestones

- [x] Parse method, URL, headers, and body flags.
- [x] Execute basic HTTP requests and print status plus body.
- [x] Add JSON or header-only output modes.
- [x] Handle timeouts and non-2xx responses clearly.
- [x] Add tests for request construction and response formatting helpers.

## Extra

- [x] Add form request support.

## Status

Completed.

## Specification

- Goal: issue basic HTTP requests with explicit, inspectable output.
- Inputs: URL plus method, headers, body, form, timeout, JSON, or header-only flags.
- Output: status, response headers, and body; JSON mode validates the response.
- Errors: report invalid flags, request failures, timeout failures, non-2xx status,
  and invalid JSON responses clearly.
- Non-goals: multipart uploads, retries, authentication flows, and streaming files.
- Acceptance: request parsing, form handling, invalid-input, and formatting tests
  pass with strict Clippy enabled.

## Change record

- Implemented async reqwest execution, request options, URL-encoded forms,
  timeout/status handling, JSON validation, and focused helper tests.

## Tips

- Keep request-building separate from presentation.
- Response formatting should not hide status or headers unexpectedly.
