// example must have main function
// Goal: Architecture

// Build:

// ```bash
// cargo run --bin api_gateway
// curl -v http://localhost:8080/orders --header "authorization: ABC" --header "X-Forwarded-For: 1.1.1.1"
// ```

// Learn:
//
// - API Gateway pattern — single entry point that routes requests to upstream services; clients only know the gateway
// - Route matching — select an upstream based on URL prefix; more specific routes win over generic ones
// - Route specificity — deeper paths beat shallower paths; static segments beat param segments at equal depth
// - Parameterized routes — a segment starting with `:` matches any path segment value
// - Catch-all routes — `/` has score (0, 0) so it is always the lowest-priority fallback
// - middleware — tower `Layer`/`Service` traits; each layer wraps the inner service to intercept requests/responses
// - resilience — retry with exponential back-off; timeout cancels a slow upstream; rate limiting caps requests per window
//
// Notes:
//
// 1. Named lifetime `'routes` — the returned `&Route` borrows from the `routes` slice, not
//    from `path`; naming the lifetime on the slice makes this explicit and lets the compiler
//    verify the borrow scope at each call site.
// 2. Score tuple `(prefix_segs.len(), static_count)` — Rust tuple comparison is lexicographic,
//    so depth dominates: a deeper route always beats a shallower one. Static count breaks ties
//    at equal depth, so "/users/admin" beats "/users/:id" for path "/users/admin".
// 3. `best_score >= score` guard — keeps the first configured route when two routes score
//    identically. Changing to `>` would make the last route win instead.
//
// Extra:
//
// - [x] Route matching
// - [x] Route specificity resolution
// - [x] Parameterized route matching
// - [] Basic API Gateway
// - [x] Prefix-based routing
// - [ ] Request forwarding
// - [ ] Header filtering (hop-by-hop header stripping)
// - [ ] Upstream error handling (502 Bad Gateway)
// - [ ] Integration tests (echo server, proxy forwarding)
// - [ ] Middleware pipeline (logging, authentication, authorization)
// - [ ] OpenTelemetry tracing — propagate trace context to upstream services
// - [ ] Retry with exponential backoff
// - [ ] Circuit breaker
// - [ ] Request timeout
// - [ ] Rate limiting
// - [ ] Load balancing across multiple upstream instances
// - [ ] Service discovery (Consul, Eureka, Kubernetes)
// - [ ] Response caching
// - [ ] JWT authentication
// - [ ] Request/response metrics (Prometheus)
// - [ ] Distributed tracing (Jaeger/Tempo)
// - [ ] Health checks
// - [ ] Graceful shutdown
// - [ ] Dynamic route configuration

use axum::{
    Router,
    extract::State,
    http::{HeaderMap, HeaderName, Request, StatusCode},
    response::Response,
    routing::any,
};
use std::time::Duration;
use std::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};
use tokio::net::TcpListener;
use tower::{Layer, Service};
use tower_governor::{
    GovernorLayer, governor::GovernorConfigBuilder, key_extractor::SmartIpKeyExtractor,
};
use tower_http::{timeout::TimeoutLayer, trace::TraceLayer};

#[derive(Clone, Copy)]
enum Env {
    #[allow(unused)]
    Test,
    Prod,
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
struct Route {
    prefix: String,
    upstream: String,
}

#[derive(Clone)]
struct AppState {
    routes: Vec<Route>,
    client: reqwest::Client,
}

struct HopByHopFilter;

trait HeaderFilter {
    fn filter(&self, headers: &HeaderMap) -> HeaderMap;
}

impl HeaderFilter for HopByHopFilter {
    // HTTP allows repeated headers
    // Set-Cookie: a=1
    // Set-Cookie: b=2
    // Internally optimized storage:
    // ("Set-Cookie", "a=1")
    // (None, "b=2")
    // Meaning:
    // “same header as previous entry”
    // So Rust uses:
    // Option<HeaderName> if destructure name from headers and
    // it is not match insert signature for key
    // HeaderMap implements FromIterator
    fn filter(&self, headers: &HeaderMap) -> HeaderMap {
        headers
            // you get references
            // no ownership transfer
            // zero cost traversal
            .iter()
            .filter(|(name, _)| !is_hop_by_hop(name))
            .map(|(name, value)| (name.clone(), value.clone()))
            .collect()
    }
}

#[derive(Clone)]
pub struct AuthService<S> {
    inner: S,
}

impl<S, B> Service<Request<B>> for AuthService<S>
where
    S: Service<Request<B>, Response = Response<axum::body::Body>> + Clone + Send + 'static,
    S::Future: Send + 'static,
    B: Send + 'static,
{
    type Response = Response<axum::body::Body>;
    type Error = S::Error;

    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, req: Request<B>) -> Self::Future {
        let mut inner = self.inner.clone();

        Box::pin(async move {
            // 🔐 auth check
            if req.headers().get("authorization").is_none() {
                return Ok(Response::builder()
                    .status(StatusCode::UNAUTHORIZED)
                    .body(axum::body::Body::empty())
                    .unwrap());
            }

            // forward request
            inner.call(req).await
        })
    }
}

// AuthLayer (Authentication / Authorization)
// Purpose

// Blocks unauthenticated requests early.

// Typical responsibilities
// Validate JWT / API key
// Extract user identity
// Attach claims to request extensions
// Reject unauthorized requests with 401/403
#[derive(Clone)]
pub struct AuthLayer;

impl<S> Layer<S> for AuthLayer {
    type Service = AuthService<S>;

    fn layer(&self, inner: S) -> Self::Service {
        AuthService { inner }
    }
}

// Selects the best matching route for `path` using prefix matching with specificity scoring.
//
// Matching rules:
//   - Split both sides by '/' (dropping empty strings from the leading slash).
//   - Path must have at least as many segments as the prefix.
//   - A prefix segment starting with ':' is a parameter — it matches any path segment.
//   - All other prefix segments must match the path segment exactly.
//
// Score: (segment_count, static_count).
//   - More segments = more specific, so a deeper route wins a shallower one.
//   - At equal depth, more static segments beat more param segments —
//     "/users/admin" wins over "/users/:id" for path "/users/admin".
//     This mirrors specificity-based routers such as axum/matchit and Fastify.
//   - The root prefix "/" has 0 segments and 0 static segments, so it is always the
//     lowest-scoring match — a natural catch-all fallback.
fn match_route<'routes>(path: &str, routes: &'routes [Route]) -> Option<&'routes Route> {
    // 'routes is a named lifetime — the returned &Route is a reference into the `routes` slice,
    // not into `path`. Naming it here tells the compiler: "the returned reference lives as long
    // as `routes` does", so it can verify that at every call site.

    // Tracks the best match found so far. None = no match yet.
    // When a candidate is found, stores Some((score, &route)) where score = (depth, static_count).
    let mut best: Option<((usize, usize), &Route)> = None;

    for route in routes {
        // "/users/admin".split('/') produces ["", "users", "admin"] because the leading '/'
        // creates an empty string before the first segment. filter removes that empty string,
        // leaving ["users", "admin"]. Same treatment for the incoming path.
        let prefix_segs: Vec<&str> = route.prefix.split('/').filter(|s| !s.is_empty()).collect();
        let path_segs: Vec<&str> = path.split('/').filter(|s| !s.is_empty()).collect();

        // A prefix with 2 segments can't match a path with only 1 segment — skip early.
        if path_segs.len() < prefix_segs.len() {
            continue;
        }

        // zip pairs up segments side by side: ("users","users"), (":id","42"), …
        // all() returns true only if every pair satisfies the closure:
        //   p.starts_with(':') — prefix segment is a param like :id, accepts any value
        //   p == s             — static segment must match exactly
        // If any pair fails, matched = false and we skip this route.
        let matched = prefix_segs
            .iter()
            .zip(path_segs.iter())
            .all(|(p, s)| p.starts_with(':') || p == s);
        if !matched {
            continue;
        }

        // Count how many prefix segments are static (not params).
        // Score is a tuple (depth, static_count):
        //   "/users/admin" → (2, 2)   "/users/:id" → (2, 1)   "/users" → (1, 1)   "/" → (0, 0)
        let static_count = prefix_segs.iter().filter(|s| !s.starts_with(':')).count();
        let score = (prefix_segs.len(), static_count);

        // Rust tuple comparison is lexicographic: (2,2) > (2,1) > (1,1) > (0,0).
        // So deeper routes beat shallower ones; at equal depth, more-static beats more-param.
        //
        // If best already has a score >= this candidate — do nothing (keep the existing best).
        // The >= (not >) means: on a tie, the first configured route in the slice wins.
        // Otherwise (_) — this candidate is better, replace best.
        match best {
            Some((best_score, _)) if best_score >= score => {}
            _ => best = Some((score, route)),
        }
    }

    // Discard the score tuple; return just the &Route.
    // If best is still None (no route matched at all), returns None.
    best.map(|(_, route)| route)
}

// RFC 9110 says these hop-by-hop headers must NOT be forwarded
fn is_hop_by_hop(name: &HeaderName) -> bool {
    matches!(
        name.as_str().to_ascii_lowercase().as_str(),
        "connection"
            | "keep-alive"
            | "proxy-authenticate"
            | "proxy-authorization"
            | "te"
            | "trailer"
            | "transfer-encoding"
            | "upgrade"
    )
}

async fn proxy_handler(
    State(state): State<AppState>,
    req: Request<axum::body::Body>,
) -> Result<Response, axum::http::StatusCode> {
    let (parts, body) = req.into_parts();

    let method = parts.method;
    let filter = HopByHopFilter;
    let forwarded_headers = filter.filter(&parts.headers);

    let uri = parts.uri;
    // let path = req.uri().path();
    let path = uri.path();
    println!("Incoming path: {}", path);

    let route = match_route(path, &state.routes).ok_or(StatusCode::NOT_FOUND)?;

    let stream = body.into_data_stream();
    // NOTE: reqwest features need to include stream
    let reqwest_body = reqwest::Body::wrap_stream(stream);
    // NOTE: path_and_query() no need query feature, but query() needs
    let path_and_query = uri.path_and_query().map(|pq| pq.as_str()).unwrap_or(path);

    // let url = format!("{}{}", route.upstream, path);
    // query() expects something serializable, such as: .query(&[("foo", "bar")])
    let url = format!("{}{}", route.upstream, path_and_query);
    println!("Redirect path and query: {}", url);

    // forwards only method and URL but not request body, query string, headers
    let resp = state
        .client
        .request(method, url)
        .headers(forwarded_headers)
        .body(reqwest_body)
        .send()
        .await
        .map_err(|_| StatusCode::BAD_GATEWAY)?;

    let resp_status = resp.status();
    let resp_headers = resp.headers().clone();

    // resp.bytes_stream()
    // resp.bytes(self) but not bytes(&self) hence it will consume the resp
    // let body = resp.bytes().await.map_err(|_| StatusCode::BAD_GATEWAY)?;
    let resp_stream = resp.bytes_stream();
    // Upstream → chunk → gateway → chunk → client
    // no more full body in RAM
    let resp_body = axum::body::Body::from_stream(resp_stream);

    // only preserve status, but missing
    // Content-Type
    // Content-Length
    // Cache-Control
    // ETag
    // Location
    // let builder = Response::builder().status(status);
    let mut response = Response::builder()
        .status(resp_status)
        .body(resp_body)
        .unwrap();

    // HeaderMap supports extend
    response.headers_mut().extend(filter.filter(&resp_headers));

    Ok(response)
}

fn build_app(routes: Vec<Route>, enable_auth: bool, env: Env) -> Router {
    // reqwest::Client::new() - connection pool, keep-alive support, DNS cache, HTTP/1.1 and HTTP/2 support
    // build with protection hung upstreams, slow DNS, socket exhaustion, excessive connection creation
    let client = reqwest::Client::builder()
        .connect_timeout(Duration::from_secs(5))
        .timeout(Duration::from_secs(30))
        .pool_max_idle_per_host(32)
        .pool_idle_timeout(Duration::from_secs(90))
        .build()
        .unwrap();
    let router = Router::new()
        // gateway already has its own router hence use single catch-all route, /*path
        // Path segments must not start with `*`. For wildcard capture, use `{*wildcard}`.
        // If you meant to literally match a segment starting with an asterisk,
        // call `without_v07_checks` on the router.
        .route("/{*path}", any(proxy_handler))
        // .layer(tower_http::trace::TraceLayer::new_for_http())
        // .layer(tower_http::timeout::TimeoutLayer::new(Duration::from_secs(30)))
        // with_state here will pass to route handler, proxy_handler 1st argument State
        // not using global variables, harder to test, hidden dependencies, difficult
        // to swap configurations
        // not capture routes in a closure,
        // .route("/*path", any(move |req| async move {, messy if State larger
        .with_state(AppState { routes, client });

    let router = if enable_auth {
        router.layer(AuthLayer)
    } else {
        router
    };

    // trace includes auth decision context after AuthLayer
    let trace_layer = TraceLayer::new_for_http()
        // install tracing for &tracing
        .on_request(|req: &axum::http::Request<_>, _span: &tracing::Span| {
            tracing::info!(
                method = %req.method(),
                uri = %req.uri(),
                "incoming request"
            );
        });
    let router = router.layer(trace_layer);

    // NOTE: test might use same identifier and burst request in short duration
    let router = match env {
        Env::Test => router, // ❌ no rate limit
        _ => {
            // TODO: ERROR:  Unable To Extract Key!
            // default PeerIpKeyExtractor
            // cannot use tower::limit::RateLimitLayer; because it has no Clone needed by router.layer
            let governor_conf = GovernorConfigBuilder::default()
                // SmartIpKeyExtractor tries, in order:
                // X-Forwarded-For
                // X-Real-IP
                // Forwarded
                // Peer socket IP
                .key_extractor(SmartIpKeyExtractor)
                .per_second(100)
                .burst_size(200)
                .finish()
                .unwrap();

            let governor_layer = GovernorLayer::new(governor_conf);
            router.layer(governor_layer)
        }
    };

    router.layer(TimeoutLayer::with_status_code(
        StatusCode::GATEWAY_TIMEOUT,
        Duration::from_secs(30),
    ))
}

#[tokio::main]
async fn main() {
    let routes = vec![
        Route {
            prefix: "/users".into(),
            upstream: "http://localhost:3001".into(),
        },
        Route {
            prefix: "/orders".into(),
            upstream: "http://localhost:3002".into(),
        },
    ];

    let app = build_app(routes, true, Env::Prod);

    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    // Bind IPv6 only: rely on dual-stack support Simple, but OS-dependent
    // *   Trying [::1]:8080...
    // * connect to ::1 port 8080 from ::1 port 55554 failed: Connection refused
    // IPv4 0.0.0.0:8080
    let listener = TcpListener::bind("[::]:8080").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{
        Router,
        body::{Body, to_bytes},
        extract::State,
        http::{HeaderValue, Method, StatusCode},
        routing::any,
    };
    use std::sync::{Arc, Mutex};
    use tokio::{net::TcpListener, sync::oneshot};
    // use tower::ServiceExt;

    #[derive(Debug)]
    struct SeenRequest {
        method: Method,
        path_and_query: String,
        trace_id: Option<String>,
        body: Vec<u8>,
        headers: HeaderMap,
    }

    #[derive(Clone)]
    struct UpstreamState {
        sender: Arc<Mutex<Option<oneshot::Sender<SeenRequest>>>>,
    }

    async fn upstream_handler(
        State(state): State<UpstreamState>,
        req: Request<axum::body::Body>,
    ) -> Response {
        let (parts, body) = req.into_parts();
        let bytes = to_bytes(body, usize::MAX).await.unwrap();

        let seen = SeenRequest {
            method: parts.method,
            path_and_query: parts
                .uri
                .path_and_query()
                .map(|pq| pq.as_str().to_string())
                .unwrap_or_else(|| parts.uri.path().to_string()),
            trace_id: parts
                .headers
                .get("x-trace-id")
                .and_then(|v| v.to_str().ok())
                .map(str::to_owned),
            body: bytes.to_vec(),
            headers: parts.headers.clone(),
        };

        if let Some(sender) = state.sender.lock().unwrap().take() {
            let _ = sender.send(seen);
        }

        Response::builder()
            .status(StatusCode::CREATED)
            .header("x-upstream", "present")
            .header("content-type", "text/plain")
            .body(Body::from("upstream ok"))
            .unwrap()
    }

    async fn spawn_upstream(
        sender: oneshot::Sender<SeenRequest>,
    ) -> (String, tokio::task::JoinHandle<()>) {
        let app = Router::new()
            .route("/{*path}", any(upstream_handler))
            .with_state(UpstreamState {
                sender: Arc::new(Mutex::new(Some(sender))),
            });

        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();
        let handle = tokio::spawn(async move {
            axum::serve(listener, app).await.unwrap();
        });

        (format!("http://{}", addr), handle)
    }

    async fn spawn_gateway(
        routes: Vec<Route>,
        enable_auth: bool,
    ) -> (String, tokio::task::JoinHandle<()>) {
        let app = build_app(routes, enable_auth, Env::Test);
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();
        let handle = tokio::spawn(async move {
            axum::serve(listener, app).await.unwrap();
        });

        (format!("http://{}", addr), handle)
    }

    fn route(prefix: &str, upstream: &str) -> Route {
        Route {
            prefix: prefix.to_string(),
            upstream: upstream.to_string(),
        }
    }

    fn table() -> Vec<Route> {
        vec![
            route("/", "http://root"),
            route("/users", "http://users-svc"),
            route("/users/admin", "http://admin-svc"),
            route("/orders", "http://orders-svc"),
        ]
    }

    #[test]
    fn exact_match() {
        let routes = table();
        assert_eq!(
            match_route("/orders", &routes).map(|r| r.upstream.as_str()),
            Some("http://orders-svc")
        );
    }

    #[test]
    fn longer_prefix_wins_over_shorter() {
        // "/users/admin/1" matches both "/users" and "/users/admin"; the longer one must win.
        let routes = table();
        assert_eq!(
            match_route("/users/admin/1", &routes).map(|r| r.upstream.as_str()),
            Some("http://admin-svc")
        );
    }

    #[test]
    fn shorter_prefix_used_when_no_longer_match() {
        // "/users/42" matches "/users" but not "/users/admin".
        let routes = table();
        assert_eq!(
            match_route("/users/42", &routes).map(|r| r.upstream.as_str()),
            Some("http://users-svc")
        );
    }

    #[test]
    fn root_catch_all() {
        // "/unknown" only matches "/".
        let routes = table();
        assert_eq!(
            match_route("/unknown", &routes).map(|r| r.upstream.as_str()),
            Some("http://root")
        );
    }

    #[test]
    fn no_match_returns_none() {
        let routes = vec![route("/api", "http://api")];
        assert!(match_route("/other", &routes).is_none());
    }

    #[test]
    fn partial_segment_does_not_match_longer_prefix() {
        // "/users/administrator" starts with "/users/admin" byte-for-byte, but "admin" is not
        // a complete segment — the next char is 'i', not '/'. Must fall back to "/users".
        let routes = table();
        assert_eq!(
            match_route("/users/administrator", &routes).map(|r| r.upstream.as_str()),
            Some("http://users-svc")
        );
    }

    #[test]
    fn param_segment_matches_any_value() {
        let routes = vec![route("/users/:id", "http://users-svc")];

        assert_eq!(
            match_route("/users/42", &routes).map(|r| r.upstream.as_str()),
            Some("http://users-svc")
        );
        assert_eq!(
            match_route("/users/abc", &routes).map(|r| r.upstream.as_str()),
            Some("http://users-svc")
        );
        assert!(match_route("/users", &routes).is_none());
    }

    #[test]
    fn static_beats_param_at_same_depth() {
        let routes = vec![
            route("/users/:id", "http://users-svc"),
            route("/users/admin", "http://admin-svc"),
        ];

        assert_eq!(
            match_route("/users/admin", &routes).map(|r| r.upstream.as_str()),
            Some("http://admin-svc")
        );
        assert_eq!(
            match_route("/users/42", &routes).map(|r| r.upstream.as_str()),
            Some("http://users-svc")
        );
    }

    #[test]
    fn equal_specificity_keeps_first_configured_route() {
        let routes = vec![
            route("/users/:id", "http://users-by-id"),
            route("/users/:name", "http://users-by-name"),
        ];

        assert_eq!(
            match_route("/users/alice", &routes).map(|r| r.upstream.as_str()),
            Some("http://users-by-id")
        );
    }

    #[test]
    fn trailing_slash_matches() {
        let routes = vec![route("/users", "http://users")];

        // /users/ same to /users
        assert_eq!(
            match_route("/users/", &routes).map(|r| r.upstream.as_str()),
            Some("http://users")
        );
    }

    #[test]
    fn root_matches_root() {
        let routes = vec![route("/", "http://root")];

        assert_eq!(
            match_route("/", &routes).map(|r| r.upstream.as_str()),
            Some("http://root")
        );
    }

    #[test]
    fn multiple_params_match() {
        let routes = vec![route("/users/:id/orders/:order_id", "http://svc")];

        assert!(match_route("/users/123/orders/456", &routes).is_some());
    }

    #[test]
    fn static_beats_param_deeply() {
        let routes = vec![
            route("/users/:id/orders", "http://param"),
            route("/users/admin/orders", "http://admin"),
        ];

        assert_eq!(
            match_route("/users/admin/orders", &routes).map(|r| r.upstream.as_str()),
            Some("http://admin")
        );
    }

    #[tokio::test]
    async fn proxy_forwards_method_query_headers_body_and_response_headers() {
        let (tx, rx) = oneshot::channel();
        let (upstream_url, upstream_handle) = spawn_upstream(tx).await;
        let (gateway_url, gateway_handle) =
            spawn_gateway(vec![route("/users", &upstream_url)], false).await;

        let response = reqwest::Client::new()
            .post(format!("{}/users/123?expand=true", gateway_url))
            .header("x-trace-id", "abc-123")
            .body("hello gateway")
            .send()
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::CREATED);
        assert_eq!(
            response.headers().get("x-upstream"),
            Some(&HeaderValue::from_static("present"))
        );
        assert_eq!(response.text().await.unwrap(), "upstream ok");

        let seen = tokio::time::timeout(std::time::Duration::from_secs(2), rx)
            .await
            .unwrap()
            .unwrap();
        assert_eq!(seen.method, Method::POST);
        assert_eq!(seen.path_and_query, "/users/123?expand=true");
        assert_eq!(seen.trace_id.as_deref(), Some("abc-123"));
        assert_eq!(seen.body, b"hello gateway");

        gateway_handle.abort();
        upstream_handle.abort();
    }

    #[tokio::test]
    async fn proxy_returns_not_found_when_no_route_matches() {
        let (gateway_url, gateway_handle) =
            spawn_gateway(vec![route("/users", "http://127.0.0.1:1")], false).await;

        let response = reqwest::Client::new()
            .get(format!("{}/orders/123", gateway_url))
            .send()
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::NOT_FOUND);

        gateway_handle.abort();
    }

    #[tokio::test]
    async fn proxy_returns_bad_gateway_when_upstream_is_unreachable() {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();
        drop(listener);

        let (gateway_url, gateway_handle) =
            spawn_gateway(vec![route("/users", &format!("http://{}", addr))], false).await;

        let response = reqwest::Client::new()
            .get(format!("{}/users/123", gateway_url))
            .send()
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::BAD_GATEWAY);

        gateway_handle.abort();
    }

    #[tokio::test]
    async fn proxy_filters_hop_by_hop_request_headers() {
        let (tx, rx) = oneshot::channel();
        let (upstream_url, upstream_handle) = spawn_upstream(tx).await;

        let (gateway_url, gateway_handle) =
            spawn_gateway(vec![route("/users", &upstream_url)], false).await;

        let _ = reqwest::Client::new()
            .get(format!("{}/users", gateway_url))
            .header("x-trace-id", "abc")
            .header("connection", "keep-alive") // ❌ should be removed
            .header("transfer-encoding", "chunked") // ❌ should be removed
            .send()
            .await
            .unwrap();

        let seen = rx.await.unwrap();

        // ✅ allowed header
        assert_eq!(seen.trace_id.as_deref(), Some("abc"));

        // ❌ hop-by-hop headers must NOT reach upstream
        assert!(seen.headers.get("connection").is_none());
        assert!(seen.headers.get("transfer-encoding").is_none());

        gateway_handle.abort();
        upstream_handle.abort();
    }

    #[test]
    fn test_filter_headers_removes_hop_by_hop() {
        let mut headers = HeaderMap::new();

        headers.insert("x-ok", "1".parse().unwrap());
        headers.insert("connection", "keep-alive".parse().unwrap());
        headers.insert("transfer-encoding", "chunked".parse().unwrap());

        let filter = HopByHopFilter;
        let filtered = filter.filter(&headers);

        assert!(filtered.get("x-ok").is_some());
        assert!(filtered.get("connection").is_none());
        assert!(filtered.get("transfer-encoding").is_none());
    }
}
