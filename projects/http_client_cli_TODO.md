# TODO: http_client_cli (⭐ 4/10)

## Usage

```bash
cargo run --bin http_client_cli
cargo test --bin http_client_cli
```

## Milestones

- [ ] Parse method, URL, headers, and body flags.
- [ ] Execute basic HTTP requests and print status plus body.
- [ ] Add JSON or header-only output modes.
- [ ] Handle timeouts and non-2xx responses clearly.
- [ ] Add tests for request construction and response formatting helpers.

## Extra

- [ ] Add form or multipart request support.

## Tips

- Keep request-building separate from presentation.
- Response formatting should not hide status or headers unexpectedly.
