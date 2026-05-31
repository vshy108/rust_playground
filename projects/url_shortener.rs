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
// 2. GET /:code: Path(code) extracts the path segment as an owned String. .get(&code)
//    returns Option<&UrlEntry>. Both match arms call .into_response() to unify the return
//    type — Redirect and StatusCode are different types, wrapping both satisfies impl IntoResponse.
//    303 See Other with Location header is the correct redirect status for GET requests.

// Extra:
//
// - [ ] expiration

use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use serde::{Deserialize, Serialize};

use axum::{
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Redirect},
    routing::{get, post},
};
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
//
// NOTE: this store is purely in-memory. All short codes are lost when the
// process exits. For persistence, write entries to a file or external DB.
type Store = Arc<Mutex<HashMap<String, UrlEntry>>>;

// POST /shorten handler.
// State(store): axum extracts the shared Store from the router's app state.
// Json(payload): axum deserialises the request body JSON into ShortenRequest.
// Returns (StatusCode::CREATED, Json(...)): axum accepts a tuple of (status, body).
// .lock().unwrap(): acquires the Mutex guard; guard is dropped at end of statement,
// releasing the lock before the response is returned.
//
// Handler parameter syntax — `State(store): State<Store>`:
//   This is Rust destructuring in function params. `State<Store>` is the wrapper
//   type axum provides; `State(store)` unpacks the inner value so `store` is already
//   `Arc<Mutex<...>>`. Without it: `state: State<Store>` and then `state.0` to access.
//   Same for `Json(payload)` — gives `ShortenRequest` directly, not `Json<ShortenRequest>`.
//
// How axum injects these automatically:
//   axum uses the `FromRequestParts` / `FromRequest` traits. Any type that implements
//   them can appear as a handler parameter. axum assembles them from the request and
//   calls the handler — you never call shorten(...) yourself.
//   .with_state(store) on the router stores an Arc clone; axum clones it per request.
async fn shorten(
    State(store): State<Store>,
    Json(payload): Json<ShortenRequest>,
) -> impl IntoResponse {
    // Uuid::new_v4().to_string() → owned String e.g. "48ecce4a-1234-..."
    // [..8] slices the first 8 chars → &str (borrowed, not owned)
    // .to_string() converts &str back to owned String — needed because `code`
    // is stored in the HashMap and returned in the response, both require ownership.
    let code = Uuid::new_v4().to_string()[..8].to_string();
    // store.lock()   — acquires the Mutex; blocks until no other handler is writing.
    //                   returns LockResult<MutexGuard<HashMap>>
    // .unwrap()       — unwraps LockResult; panics only if another thread panicked
    //                   while holding the lock ("poisoned mutex") — acceptable here.
    // .insert(k, v)   — adds the entry to the HashMap.
    // code.clone()    — insert takes ownership of the key; we clone because `code`
    //                   is also used on the next line for the response.
    // payload.url     — moved into UrlEntry (String is not Copy, so no clone needed).
    // MutexGuard dropped at end of statement → lock released before response is built.
    store.lock().unwrap().insert(
        code.clone(),
        UrlEntry {
            original_url: payload.url,
        },
    );
    (StatusCode::CREATED, Json(ShortenResponse { code }))
}

// GET /:code handler.
// Path(code): axum extracts the {code} path segment as an owned String.
// .get(&code): looks up the code in the HashMap, returns Option<&UrlEntry>.
// Both match arms call .into_response() to produce a uniform Response type —
// Redirect and StatusCode are different types; .into_response() erases the
// difference so the function can return a single impl IntoResponse.
// 303 See Other: correct redirect status for GET — tells the client to GET the
// new URL. Location header carries the original URL.
async fn redirect(State(store): State<Store>, Path(code): Path<String>) -> impl IntoResponse {
    let map = store.lock().unwrap();
    match map.get(&code) {
        Some(entry) => Redirect::to(&entry.original_url).into_response(),
        None => StatusCode::NOT_FOUND.into_response(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{
        body::Body,
        http::Request,
    };
    use tower::ServiceExt; // .oneshot(): send one request to a Router without a TCP server

    // Builds a fresh Router with an empty in-memory store.
    // Calling this in each test keeps tests independent — no shared state between them.
    fn make_app() -> Router {
        let store: Store = Arc::new(Mutex::new(HashMap::new()));
        Router::new()
            .route("/shorten", post(shorten))
            .route("/{code}", get(redirect))
            .with_state(store)
    }

    // POST /shorten should return 201 Created and a non-empty short code.
    #[tokio::test]
    async fn post_shorten_returns_201_and_code() {
        let response = make_app()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/shorten")
                    .header("content-type", "application/json")
                    .body(Body::from(r#"{"url":"https://example.com"}"#))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::CREATED);
        let bytes = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap();
        let v: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
        // The code field must exist and be non-empty.
        assert!(!v["code"].as_str().unwrap_or("").is_empty());
    }

    // GET /:code with a known code should return a 3xx redirect pointing to the original URL.
    #[tokio::test]
    async fn get_known_code_redirects() {
        // Seed the store directly so we control the exact code — no HTTP round-trip needed.
        let store: Store = Arc::new(Mutex::new(HashMap::new()));
        store.lock().unwrap().insert(
            "testcode".to_string(),
            UrlEntry {
                original_url: "https://example.com".to_string(),
            },
        );
        let app = Router::new()
            .route("/shorten", post(shorten))
            .route("/{code}", get(redirect))
            .with_state(store);

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/testcode")
                    .body(Body::from(""))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert!(response.status().is_redirection());
        assert_eq!(response.headers()["location"], "https://example.com");
    }

    // GET /:code with an unknown code should return 404 Not Found.
    #[tokio::test]
    async fn get_unknown_code_returns_404() {
        let response = make_app()
            .oneshot(
                Request::builder()
                    .uri("/doesnotexist")
                    .body(Body::from(""))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }
}

#[tokio::main]
async fn main() {
    let store: Store = Arc::new(Mutex::new(HashMap::new()));
    let app = Router::new()
        .route("/shorten", post(shorten))
        .route("/{code}", get(redirect))
        .with_state(store);
    // TcpListener::bind: asks the OS to reserve port 3000. Async because the OS
    // call can take a moment (checking port availability, allocating the socket).
    // .await suspends main until the port is ready; .unwrap() panics if port is in use.
    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    // axum::serve: starts the accept loop — accept connection → spawn task → handle.
    // Never returns (runs until the process is killed). .await hands control to tokio
    // so other tasks can run while the server waits for the next connection.
    // Without .await, serve() returns a Future — nothing runs until it is awaited.
    axum::serve(listener, app).await.unwrap();
}
