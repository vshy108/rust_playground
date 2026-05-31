// Goal: Data modeling
//
// API:
//   POST /shorten   body: { "url": "https://..." }
//                   resp: { "code": "abc123" }
//   GET  /:code     redirect to original URL, or 404

// Learn:
//
// - HashMap
//   - HashMap<K, V> maps keys to values with O(1) average lookup.
//   - Use .insert(k, v) to add, .get(&k) to look up (returns Option<&V>).
//
// - Arc<Mutex<HashMap>>: shared mutable state across handlers
//   - A plain HashMap in main has one owner; passing it to a handler moves it,
//     so the other handler can't use it.
//   - Arc (shared pointer) lets both handlers point to the same HashMap in memory.
//     When all Arc clones are dropped, the HashMap is freed.
//   - Mutex ensures only one handler reads/writes at a time.
//     .lock().unwrap() returns a MutexGuard — the guard releases the lock when dropped.
//   - Together: Arc<Mutex<HashMap>> = shared ownership + safe concurrent access.
//
// - std::sync::Mutex vs tokio::sync::Mutex
//   - std::sync::Mutex: blocks the OS thread while waiting for the lock.
//     Cannot be held across .await — compiler rejects it (guard is not Send).
//     Use when: lock, do quick sync work (insert/get), drop immediately.
//   - tokio::sync::Mutex: async-aware — .lock().await suspends the task, not the
//     thread, so other tasks can run while waiting. Can be held across .await.
//     Use when: you need to do async work while holding the lock.
//   - Rule: this project uses std::sync::Mutex because .insert() and .get() are
//     instant sync operations — no awaiting inside the lock.
//
// - structs
//   - Structs group related fields under a named type.
//   - #[derive(Deserialize)] lets serde parse incoming JSON into a struct.
//   - #[derive(Serialize)] lets serde turn a struct into outgoing JSON.
//
// - UUID
//   - Uuid::new_v4() generates a random 128-bit identifier.
//   - .to_string() gives the full 36-char hyphenated form; take the first 8 chars
//     for a compact short code.

// Progress:
// 1. POST /shorten: State(store) injects the shared Arc<Mutex<HashMap>>; Json(payload)
//    deserialises the request body. Uuid::new_v4().to_string()[..8] gives an 8-char code.
//    .lock().unwrap() acquires the Mutex — guard is dropped at end of statement, releasing
//    the lock immediately. Returns (StatusCode::CREATED, Json(...)) as a tuple — axum
//    accepts tuples of (StatusCode, impl IntoResponse) as a response.

// Extra:
//
// - [ ] expiration

use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use serde::{Deserialize, Serialize};

use axum::{Json, Router, extract::State, http::StatusCode, response::IntoResponse, routing::post};
use tokio::net::TcpListener;
use uuid::Uuid;

// UrlEntry is the value stored in the HashMap for each short code.
// Lives only in server memory — never serialised to JSON.
// Extra: add `expires_at: Option<std::time::Instant>` here for expiration.
struct UrlEntry {
    original_url: String,
}

// ShortenRequest is parsed from the POST /shorten request body.
// #[derive(Deserialize)]: serde reads {"url":"https://..."} → ShortenRequest { url }.
#[derive(Deserialize)]
struct ShortenRequest {
    url: String,
}

// ShortenResponse is serialised into the POST /shorten response body.
// #[derive(Serialize)]: serde writes ShortenResponse { code } → {"code":"abc123"}.
#[derive(Serialize)]
struct ShortenResponse {
    code: String,
}

// Store is a type alias for the shared HashMap wrapped in Arc<Mutex>.
// Arc: shared ownership across handlers. Mutex: one writer at a time.
// Using a type alias avoids repeating Arc<Mutex<HashMap<String, UrlEntry>>> everywhere.
type Store = Arc<Mutex<HashMap<String, UrlEntry>>>;

// POST /shorten handler.
// State(store): axum extracts the shared Store from the router's app state.
// Json(payload): axum deserialises the request body JSON into ShortenRequest.
// Returns (StatusCode::CREATED, Json(...)): axum accepts a tuple of (status, body).
// .lock().unwrap(): acquires the Mutex guard; guard is dropped at end of statement,
// releasing the lock before the response is returned.
async fn shorten(
    State(store): State<Store>,
    Json(payload): Json<ShortenRequest>,
) -> impl IntoResponse {
    let code = Uuid::new_v4().to_string()[..8].to_string();
    store.lock().unwrap().insert(code.clone(), UrlEntry { original_url: payload.url });
    (StatusCode::CREATED, Json(ShortenResponse { code }))
}

#[tokio::main]
async fn main() {
    let store: Store = Arc::new(Mutex::new(HashMap::new()));
    let app = Router::new()
        .route("/shorten", post(shorten))
        .with_state(store);
    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
