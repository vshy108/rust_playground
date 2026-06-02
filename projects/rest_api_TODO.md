# TODO: rest_api

## Usage

```bash
cargo run --bin rest_api
cargo test --bin rest_api
```

## 1. Data model

- [ ] Define an `Item` struct with `id: u64`, `name: String`, `done: bool`.
- [ ] Define `CreateItem` and `UpdateItem` request structs with `#[derive(Deserialize)]`.

Acceptance check: structs compile with serde derives.

## 2. In-memory store

- [ ] Define `type Store = Arc<Mutex<HashMap<u64, Item>>>`.
- [ ] Write a `new_store()` helper that returns an empty store.

Acceptance check: store initialises and can be cloned into handlers.

## 3. CRUD handlers

- [ ] `GET /items` — return all items as JSON array.
- [ ] `POST /items` — insert a new item; return 201 with the created item.
- [ ] `GET /items/:id` — return item or 404.
- [ ] `PUT /items/:id` — update name/done or 404.
- [ ] `DELETE /items/:id` — remove item or 404.

Acceptance check: each endpoint responds correctly via `curl` or `oneshot` test.

## 4. Router + main

- [ ] Wire all handlers into a `Router` with shared `State`.
- [ ] Bind to `0.0.0.0:3000` with `tokio::net::TcpListener`.

Acceptance check: `cargo run` starts without error; `curl localhost:3000/items` returns `[]`.

## 5. Tests

- [ ] Create item returns 201 with body.
- [ ] Get unknown id returns 404.
- [ ] Update item changes the value.
- [ ] Delete item removes it from subsequent list.

## Extra: JWT

- [ ] Add a `POST /login` endpoint that returns a signed JWT.
- [ ] Add an axum middleware layer that validates the JWT on protected routes.
