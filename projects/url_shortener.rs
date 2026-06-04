// Goal: Data modeling
//
// API:
//   POST /shorten   body: { "url": "https://..." }
//                   resp: { "code": "abc123" }
//   GET  /:code     redirect to original URL, or 404

// Learn:
//
// - HashMap — key-value store with O(1) average lookup; `.insert(k, v)` to add, `.get(&k)` returns `Option<&V>`
// - Arc<Mutex<T>> — shared mutable state across handlers; Arc = shared ownership, Mutex = exclusive access
// - std::sync::Mutex vs tokio::sync::Mutex — blocking vs async-aware lock; use std when the critical section has no `.await`
// - serde — `#[derive(Deserialize, Serialize)]` on structs for JSON request/response bodies
// - axum — Router maps HTTP methods to handlers; extractors (State, Path, Json) resolve from the request
// - async / await — `async fn` returns a Future; `.await` suspends the task without blocking the thread
// - UUID — `Uuid::new_v4()` generates a random 128-bit identifier; first 8 chars for a short code

// Notes:
//
// 1. Arc<Mutex<HashMap>> — A plain HashMap in main has one owner; passing it to a handler moves it,
//    so the other handler can't use it. Arc lets both handlers point to the same HashMap in memory;
//    when all Arc clones are dropped, the HashMap is freed. Mutex ensures only one handler reads/writes
//    at a time — `.lock().unwrap()` returns a MutexGuard; the guard releases the lock when dropped.
//
// 2. std::sync::Mutex vs tokio::sync::Mutex — std::sync::Mutex blocks the OS thread while waiting;
//    cannot be held across `.await` (compiler rejects it: guard is not Send). tokio::sync::Mutex is
//    async-aware — `.lock().await` suspends the task, not the thread; can be held across `.await`.
//    Rule: use std here because `.insert()` and `.get()` are instant sync ops — no awaiting inside the lock.
//
// 3. axum extractors — State, Path, Json appear as handler parameters; axum resolves them from the
//    request automatically. State injects shared app data; Path extracts a URL segment as an owned String;
//    Json deserialises the request body. Handlers return `impl IntoResponse` — StatusCode, Json<T>,
//    Redirect, and tuples of those all implement IntoResponse.
//
// 4. async / await — `async fn` returns a Future that does nothing until polled by an executor.
//    `.await` suspends the current task until the Future resolves, yielding the thread to other tasks.
//    `#[tokio::main]` starts the tokio async runtime; `#[tokio::test]` does the same per test.
//
// 5. POST /shorten: State(store) injects the shared Arc<Mutex<HashMap>>; Json(payload)
//    deserialises the request body. Uuid::new_v4().to_string()[..8] gives an 8-char code.
//    .lock().unwrap() acquires the Mutex — guard is dropped at end of statement, releasing
//    the lock immediately. Returns (StatusCode::CREATED, Json(...)) as a tuple — axum
//    accepts tuples of (StatusCode, impl IntoResponse) as a response.
// 6. GET /:code: Path(code) extracts the path segment as an owned String. .get(&code)
//    returns Option<&UrlEntry>. Both match arms call .into_response() to unify the return
//    type — Redirect and StatusCode are different types, wrapping both satisfies impl IntoResponse.
//    axum's Redirect::to() emits 303 See Other. 303 is idiomatic for Post/Redirect/Get
//    (forces the follow-up to GET); for a plain GET shortener, 301/302 is also conventional.
// 7. Tests: tower::ServiceExt::oneshot() sends a Request directly into the Router without
//    a TCP server. Each test builds its own Store so tests are fully independent.
//    #[tokio::test] spins up a tokio runtime per test — needed because handlers are async.
// 8. Expiration (steps 2–3): ShortenRequest gained ttl_secs: Option<u64>. UrlEntry gained
//    expires_at: Option<Instant>. In shorten, payload.ttl_secs.map(|secs| Instant::now() +
//    Duration::from_secs(secs)) computes the deadline — .map() preserves the Option shape
//    without if/else. std::time::Instant is monotonic (never jumps) — safe for in-process
//    deadline comparison. Duration imported alongside Instant from std::time.

// Extra:
//
// - [x] expiration steps 2–3 (data model + shorten handler)
// - [x] expiration step 4 (redirect: check deadline, return 410 Gone)
// - [x] expiration step 5 (tests: expired, non-expired, no-ttl)

use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
    time::{Duration, Instant},
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
// expires_at: None = no TTL (never expires); Some(t) = deadline as a monotonic Instant.
struct UrlEntry {
    original_url: String,
    expires_at: Option<Instant>,
}

// ShortenRequest is parsed from the POST /shorten request body.
// #[derive(Deserialize)]: serde reads {"url":"https://..."} → ShortenRequest { url }.
#[derive(Deserialize)]
struct ShortenRequest {
    url: String,
    ttl_secs: Option<u64>,
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
            // .map() on Option<u64>: Some(n) → Some(deadline), None → None.
            // Avoids if/else and unwrap — the Option shape is preserved automatically.
            expires_at: payload
                .ttl_secs
                .map(|secs| Instant::now() + Duration::from_secs(secs)),
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
// axum's Redirect::to() emits 303 See Other — idiomatic for Post/Redirect/Get.
// For a plain GET shortener, 301 (permanent) or 302 (temporary) are also conventional.
async fn redirect(State(store): State<Store>, Path(code): Path<String>) -> impl IntoResponse {
    let map = store.lock().unwrap();
    match map.get(&code) {
        Some(entry) => {
            // .map_or(false, |t| t < Instant::now()):
            //   - None      → false (no expiry set, never expired)
            //   - Some(t)   → true if deadline t is already in the past
            // Instant subtraction is not used because it panics on underflow;
            // comparing two Instants with < is always safe.
            // FIX: clippy::unnecessary_map_or — map_or(false, |t| pred(t)) is a verbose way to
            // ask "does Some satisfy this predicate?". is_some_and(pred) expresses that intent
            // directly and avoids the explicit false default.
            let expired = entry.expires_at.is_some_and(|t| t < Instant::now());
            if expired {
                StatusCode::GONE.into_response()
            } else {
                Redirect::to(&entry.original_url).into_response()
            }
        }
        None => StatusCode::NOT_FOUND.into_response(),
    }
}

// app: wires routes and shared state into a Router.
// Extracted so main and tests share the same route definitions — adding a route
// here automatically covers both the running server and all test Routers.
fn app(store: Store) -> Router {
    Router::new()
        .route("/shorten", post(shorten))
        .route("/{code}", get(redirect))
        .with_state(store)
}

#[tokio::main]
async fn main() {
    let store: Store = Arc::new(Mutex::new(HashMap::new()));
    let app = app(store);
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

#[cfg(test)]
// #[cfg(test)]: this whole module is compiled only when running `cargo test`.
// It is stripped from the release binary — no test code ships to production.
mod tests {
    use super::*;
    // use super::*: imports everything from the parent module (handlers, Store,
    // structs, type aliases). Lets tests call shorten/redirect directly without
    // re-importing each item.
    use axum::{
        body::Body,
        // Body::from("..."): wraps a byte string into an axum request body.
        http::Request,
        // Request::builder(): constructs an HTTP request without a real TCP connection.
        // Set method, URI, headers, and body, then .unwrap() to get the Request.
    };
    use tower::ServiceExt;
    // ServiceExt adds .oneshot() to any Tower Service (Router implements Service).
    // .oneshot(request): sends exactly one request into the router, returns a Future
    // that resolves to the Response. No TCP listener, no port — tests run in-process.

    // build_router: thin wrapper around the module-level app() function.
    // Keeps test helpers readable while sharing the real route definitions.
    fn build_router(store: Store) -> Router {
        app(store)
    }

    // make_app: helper that builds a Router with a fresh, empty Store.
    // Calling this per-test means each test starts with no existing short codes,
    // so tests don't interfere with each other (no shared global state).
    fn make_app() -> Router {
        build_router(Arc::new(Mutex::new(HashMap::new())))
    }

    // make_app_with_entry: builds a Router pre-seeded with one UrlEntry.
    // Used by redirect tests that need a known code — avoids repeating the
    // store-create → insert → router-build block in every test.
    fn make_app_with_entry(code: &str, expires_at: Option<Instant>) -> Router {
        let store: Store = Arc::new(Mutex::new(HashMap::new()));
        store.lock().unwrap().insert(
            code.to_string(),
            UrlEntry {
                original_url: "https://example.com".to_string(),
                expires_at,
            },
        );
        build_router(store)
    }

    // #[tokio::test]: axum handlers are async, so tests must also be async.
    // This attribute spins up a tokio runtime for the duration of one test,
    // then tears it down — equivalent to #[tokio::main] but scoped to one test.
    #[tokio::test]
    async fn post_shorten_returns_201_and_code() {
        let response = make_app()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/shorten")
                    .header("content-type", "application/json")
                    // r#"..."#: raw string literal — no need to escape inner quotes.
                    .body(Body::from(r#"{"url":"https://example.com"}"#))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::CREATED);
        // axum::body::to_bytes: consumes the response body stream into a Bytes buffer.
        // usize::MAX: no size cap — safe here because test responses are tiny.
        let bytes = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap();
        // serde_json::from_slice: deserialises raw bytes → serde_json::Value (generic JSON).
        // Using Value instead of ShortenResponse avoids coupling the test to the exact struct.
        let v: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
        assert!(!v["code"].as_str().unwrap_or("").is_empty());
    }

    #[tokio::test]
    async fn get_known_code_redirects() {
        // make_app_with_entry seeds the store directly — no HTTP round-trip to POST /shorten.
        // "testcode" is always the key, making the test deterministic (no random UUID).
        let response = make_app_with_entry("testcode", None)
            .oneshot(
                Request::builder()
                    .uri("/testcode")
                    .body(Body::from(""))
                    .unwrap(),
            )
            .await
            .unwrap();

        // .is_redirection(): true for any 3xx status (300–399).
        // Checks the class without hard-coding 303 — protects against
        // changing redirect type (301 Permanent, 302 Found, etc.) in the future.
        assert!(response.status().is_redirection());
        // Location header must point back to the original URL.
        assert_eq!(response.headers()["location"], "https://example.com");
    }

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

    // Seed a code whose deadline is 1 second in the past → already expired.
    // Instant::now() - Duration::from_secs(1) gives a deadline that is already
    // behind the current time, so the handler must return 410 Gone immediately.
    #[tokio::test]
    async fn get_expired_code_returns_410() {
        // Deadline 1 second in the past → already expired when the handler checks.
        let response =
            make_app_with_entry("expired", Some(Instant::now() - Duration::from_secs(1)))
                .oneshot(
                    Request::builder()
                        .uri("/expired")
                        .body(Body::from(""))
                        .unwrap(),
                )
                .await
                .unwrap();

        assert_eq!(response.status(), StatusCode::GONE);
    }

    // Seed a code whose deadline is 60 seconds in the future → not yet expired.
    #[tokio::test]
    async fn get_non_expired_code_redirects() {
        // Deadline 60 seconds in the future → not yet expired.
        let response = make_app_with_entry("fresh", Some(Instant::now() + Duration::from_secs(60)))
            .oneshot(
                Request::builder()
                    .uri("/fresh")
                    .body(Body::from(""))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert!(response.status().is_redirection());
    }

    // Seed a code with expires_at: None → no TTL, never expires regardless of time.
    #[tokio::test]
    async fn get_no_ttl_code_never_expires() {
        // expires_at: None → no TTL, never expires regardless of time.
        let response = make_app_with_entry("permanent", None)
            .oneshot(
                Request::builder()
                    .uri("/permanent")
                    .body(Body::from(""))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert!(response.status().is_redirection());
    }
}
