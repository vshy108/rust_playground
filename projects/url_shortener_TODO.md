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

- [x] POST /shorten returns 201 and a non-empty code.
- [x] GET /:code with a known code returns a redirect.
- [x] GET /unknown returns 404.

Acceptance check:

```bash
cargo test --bin url_shortener
```

## 4. Extra — Expiration

### Step 1: data model (`UrlEntry`)
- [x] Add `expires_at: Option<std::time::Instant>` to `UrlEntry`.
      `Option` = no expiry when `None`; `Instant` = monotonic deadline, safe for in-process comparison.

### Step 2: request model (`ShortenRequest`)
- [ ] Add `ttl_secs: Option<u64>` to `ShortenRequest`.
      `Option` = caller omits the field → no expiry. `u64` = seconds, can't be negative.

### Step 3: `shorten` handler — compute deadline
- [ ] When `ttl_secs` is `Some(n)`, set `expires_at = Some(Instant::now() + Duration::from_secs(n))`.
      When `None`, set `expires_at = None`.
      Fix existing `UrlEntry { original_url: payload.url }` — now needs `expires_at` too.

### Step 4: `redirect` handler — reject expired entries
- [x] After the `.get(&code)` `Some(entry)` arm, check if `expires_at` is set and in the past:
      `entry.expires_at.map_or(false, |t| t < Instant::now())`
      If expired → return `StatusCode::GONE` (410). Otherwise → redirect as before.

### Step 5: tests
- [x] `get_expired_code_returns_410`: seed store with `expires_at = Some(Instant::now() - Duration::from_secs(1))` (already past), GET → 410.
- [x] `get_non_expired_code_redirects`: seed with `expires_at = Some(Instant::now() + Duration::from_secs(60))`, GET → 3xx.
- [x] `get_no_ttl_code_never_expires`: seed with `expires_at = None`, GET → 3xx (never expires).

Acceptance check:

```bash
cargo test --bin url_shortener

# manual smoke test:
cargo run --bin url_shortener &
CODE=$(curl -s -X POST http://localhost:3000/shorten \
  -H 'Content-Type: application/json' \
  -d '{"url":"https://example.com","ttl_secs":5}' | jq -r .code)
curl -v http://localhost:3000/$CODE          # expect 303
sleep 6
curl -v http://localhost:3000/$CODE          # expect 410
```
