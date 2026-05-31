# TODO: url_shortener

## Usage

```bash
cargo run --bin url_shortener                    # start server on :3000
cargo run --bin url_shortener &                  # start in background (shell free)
curl -s -X POST http://localhost:3000/shorten \
  -H 'Content-Type: application/json' \
  -d '{"url":"https://example.com"}' | jq       # shorten a URL
curl -v http://localhost:3000/<code>             # follow redirect
cargo test --bin url_shortener

# Stop the server:
# Ctrl+C only kills the foreground process — background jobs (&) are unaffected.
# To kill a background server on port 3000:
lsof -ti:3000 | xargs kill -9
```

## 1. POST /shorten

- [x] Generate a short code: first 8 chars of `Uuid::new_v4().to_string()`.
- [x] Insert `UrlEntry { original_url }` into the store under the code.
- [x] Return `Json(ShortenResponse { code })` with `StatusCode::CREATED`.

Acceptance check:

```bash
cargo run --bin url_shortener &
curl -s -X POST http://localhost:3000/shorten \
  -H 'Content-Type: application/json' \
  -d '{"url":"https://example.com"}' | jq
# expected: { "code": "xxxxxxxx" }
```

## 2. GET /:code

- [x] Lock the store and call `.get(&code)`.
- [x] If found, return `Redirect::to(&entry.original_url)`.
- [x] If not found, return `StatusCode::NOT_FOUND`.

Acceptance check:

```bash
CODE=$(curl -s -X POST http://localhost:3000/shorten \
  -H 'Content-Type: application/json' \
  -d '{"url":"https://example.com"}' | jq -r .code)
curl -v http://localhost:3000/$CODE
# expected: 3xx redirect to https://example.com

curl -v http://localhost:3000/notexist
# expected: 404
```

## 3. Tests

- [ ] POST /shorten returns 201 and a non-empty code.
- [ ] GET /:code with a known code returns a redirect.
- [ ] GET /unknown returns 404.

Acceptance check:

```bash
cargo test --bin url_shortener
```

## 4. Extra

- [ ] Expiration: add `expires_at: Option<std::time::Instant>` to `UrlEntry`.
      Accept an optional `ttl_secs: u64` in `ShortenRequest`.
      On GET /:code, reject expired entries with 410 Gone.
