# TODO: rest_api (⭐ 5/10)


## Usage

```bash
cargo run --bin rest_api
cargo test --bin rest_api
```

## 1. Data model

- [x] Define an `Item` struct with `id: u64`, `name: String`, `done: bool`.
- [x] Define `CreateItem` and `UpdateItem` request structs with `#[derive(Deserialize)]`.

Acceptance check: structs compile with serde derives.

## 2. In-memory store

- [x] Define `type Store = Arc<Mutex<HashMap<u64, Item>>>`.
- [x] Write a `new_store()` helper that returns an empty store.

Acceptance check: store initialises and can be cloned into handlers.

## 3. CRUD handlers

- [x] `GET /items` — return all items as JSON array.
- [x] `POST /items` — insert a new item; return 201 with the created item.
- [x] `GET /items/:id` — return item or 404.
- [x] `PUT /items/:id` — update name/done or 404.
- [x] `DELETE /items/:id` — remove item or 404.

Acceptance check: each endpoint responds correctly via `curl` or `oneshot` test.

## 4. Router + main

- [x] Wire all handlers into a `Router` with shared `State`.
- [x] Bind to `0.0.0.0:3000` with `tokio::net::TcpListener`.

Acceptance check: `cargo run` starts without error; `curl localhost:3000/items` returns `[]`.

## 5. Tests

- [x] Create item returns 201 with body.
- [x] Get unknown id returns 404.
- [x] Update item changes the value.
- [x] Delete item removes it from subsequent list.

## Extra: JWT

- [x] Add a `POST /login` endpoint that returns a signed JWT.
- [x] Add an axum middleware layer that validates the JWT on protected routes.

## Status

Completed.

## Specification

- Goal: provide an in-memory JSON CRUD API with JWT-protected item routes.
- Inputs: JSON item requests, bearer tokens, and HTTP method/path combinations.
- Output: JSON items, status-specific CRUD responses, and signed login tokens.
- Errors: return 404 for missing items and 401 for missing or invalid tokens.
- Non-goals: persistent storage, production secret management, and multi-user identity provisioning.
- Acceptance: CRUD handler tests, sorted listing, JWT middleware compilation, and strict Clippy checks pass.

## Change record

- Implemented Axum routes, shared async state, CRUD handlers, JWT login, bearer middleware, deterministic ordering, and focused handler tests.

## Tips

- Start with protocol and contract tests first (request, response, error, timeout).
- Build a strict parser before adding convenience behavior; fail closed on malformed input.
- Add structured request logging early so debugging network paths is cheap.
- Keep connection lifecycle explicit: open, active, idle timeout, close.
- Add load and latency checks after correctness is stable.

## Learn Notes

- async — `async fn` returns a Future; `.await` suspends the task without blocking the thread
- middleware — tower layers applied to every request (logging, auth, error handling)
- axum routing — `Router::new().route(path, method(handler))` wires HTTP methods to handlers
- serde — `#[derive(Deserialize, Serialize)]` on structs for JSON request/response bodies
- State — `Arc<Mutex<T>>` shared across handlers; cloned into each via axum's `State` extractor

## Extra

- JWT authentication middleware
