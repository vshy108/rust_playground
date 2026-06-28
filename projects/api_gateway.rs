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
//
// Extra:
//
// Core Gateway
// - [x] Route matching (prefix + param + specificity resolution)
// - [x] Deterministic route scoring (depth + static priority)
// - [x] Parameterized route matching
// - [x] Catch-all route support
// - [x] Request forwarding (streaming, zero-buffer)
// - [x] Header filtering (hop-by-hop compliance)
// - [x] Response header propagation
// - [x] Upstream error handling (502 Bad Gateway)
// - [x] Upstream timeout handling (gateway-level enforcement)
// - [x] Load balancing (round-robin across upstream pools)
// - [x] Integration tests (routing, proxy, resilience, auth, timeout)
//
// Middleware & Platform
// - [x] Middleware pipeline (Tower-based layering)
// - [x] Authentication middleware (header-based auth gate)
// - [x] Rate limiting (tower_governor, IP-based)
// - [x] Request timeout (Tower TimeoutLayer)
// - [x] Request ID generation + propagation
// - [x] Structured logging (TraceLayer)
// - [x] Graceful shutdown (Ctrl+C signal handling)
// - [x] Health & readiness endpoints
//
// Observability
// - [x] Request tracing (TraceLayer hooks)
// - [ ] Metrics export (Prometheus / OpenMetrics)
// - [ ] Distributed tracing (OpenTelemetry propagation)
// - [ ] Log correlation across upstream hops
//
// Resilience
// - [ ] Retry with exponential backoff (client-side)
// - [ ] Circuit breaker per upstream (failure tracking)
// - [x] Load balancing across upstream instances
// - [ ] Bulkhead isolation (per-route concurrency limits)
//
// Service Discovery & Configuration
// - [ ] Static route config only (current state)
// - [ ] Dynamic config reload (hot reload / watch file / config server)
// - [ ] Service discovery integration (Kubernetes / Consul / Eureka)
//
// Performance
// - [x] Streaming proxy (no full-body buffering)
// - [x] Connection pooling (reqwest client reuse)
// - [ ] Response caching layer (edge / in-memory)
// - [ ] HTTP/2 upstream multiplexing optimization
//
// Security
// - [x] Authorization header validation (basic gate)
// - [ ] JWT validation (claims-based auth)
// - [ ] mTLS upstream communication
// - [ ] Request sanitization hardening
//
// Testing
// - [x] Unit tests (route matching, header filtering)
// - [x] Integration tests (proxy correctness, auth, timeout, round-robin)
// - [x] E2E tests (upstream simulation + failure injection)
// - [ ] Chaos testing (latency jitter, packet drop simulation)

#[allow(unused_imports)]
use axum::{
    Router,
    http::{HeaderMap, Request, StatusCode},
    response::Response,
    routing::{any, get},
};
use std::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};
use std::{sync::Arc, time::Duration};
use tokio::net::TcpListener;
use tower::{Layer, Service};
use tower_governor::{
    GovernorLayer, governor::GovernorConfigBuilder, key_extractor::SmartIpKeyExtractor,
};
use tower_http::request_id::{MakeRequestUuid, PropagateRequestIdLayer, SetRequestIdLayer};
use tower_http::{classify::ServerErrorsFailureClass, timeout::TimeoutLayer, trace::TraceLayer};

// mod means install the local folder as cargo dependency
mod gateway;
#[allow(unused_imports)]
use gateway::headers::HopByHopFilter;
use gateway::round_robin::LoadBalancer;
#[allow(unused_imports)]
use gateway::route_matcher::match_route;
use gateway::router::build_router;
use gateway::state::AppState;
use gateway::types::{Env, Route};

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

fn build_app(routes: Vec<Route>, enable_auth: bool, env: Env, timeout: Duration) -> Router {
    // reqwest::Client::builder() configures shared pooling, keep-alive, and timeouts for upstream calls.
    // These limits protect the gateway from slow DNS, hung upstreams, and excessive connection churn.
    let client = reqwest::Client::builder()
        .connect_timeout(Duration::from_secs(5))
        .timeout(Duration::from_secs(30))
        .pool_max_idle_per_host(32)
        .pool_idle_timeout(Duration::from_secs(90))
        .build()
        .unwrap();
    let state = AppState {
        routes,
        client,
        lb: Arc::new(LoadBalancer::new()),
    };
    let router = build_router(state);

    // Auth is layered closest to the handler so later layers can short-circuit before it when needed.
    let router = if enable_auth {
        router.layer(AuthLayer)
    } else {
        router
    };

    // NOTE: test might use same identifier and burst request in short duration
    let router = match env {
        Env::Test => router, // ❌ no rate limit
        _ => {
            // SmartIpKeyExtractor uses forwarded headers first and falls back to the peer socket IP.
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

    let router = router.layer(TimeoutLayer::with_status_code(
        StatusCode::GATEWAY_TIMEOUT,
        timeout,
    ));

    let trace_layer = TraceLayer::new_for_http()
        // install tracing for &tracing
        // Record request metadata on the tracing span.
        .on_request(|req: &axum::http::Request<_>, _span: &tracing::Span| {
            tracing::info!(
                request_id = ?req.headers().get("x-request-id"),
                method = %req.method(),
                uri = %req.uri(),
                "incoming request"
            );
        })
        .on_response(
            |response: &axum::http::Response<axum::body::Body>,
             latency: Duration,
             _span: &tracing::Span| {
                tracing::info!("response: {} {:?}", response.status(), latency)
            },
        )
        .on_failure(
            |error: ServerErrorsFailureClass, _latency: Duration, _span: &tracing::Span| {
                tracing::error!("error: {}", error)
            },
        );

    let router = router.layer(trace_layer);

    router
        .layer(SetRequestIdLayer::x_request_id(MakeRequestUuid))
        .layer(PropagateRequestIdLayer::x_request_id())
}

// graceful shutdown
async fn shutdown_signal() {
    tokio::signal::ctrl_c()
        .await
        .expect("failed to install Ctrl+C handler");
}

#[tokio::main]
async fn main() {
    // Static demo route table; each route can fan out to multiple upstream instances.
    let routes = vec![
        Route {
            // Incoming paths beginning with /users are proxied to this upstream pool.
            prefix: "/users".into(),
            upstreams: vec![
                "http://localhost:3001".into(),
                "http://localhost:3003".into(),
            ],
        },
        Route {
            // Incoming paths beginning with /orders are proxied to this upstream pool.
            prefix: "/orders".into(),
            upstreams: vec![
                "http://localhost:3002".into(),
                "http://localhost:3004".into(),
            ],
        },
    ];

    // Build production middleware stack over the configured routes.
    let app = build_app(routes, true, Env::Prod, Duration::from_secs(15));

    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    // Bind IPv6 only: rely on dual-stack support Simple, but OS-dependent
    // *   Trying [::1]:8080...
    // * connect to ::1 port 8080 from ::1 port 55554 failed: Connection refused
    // IPv4 0.0.0.0:8080
    let listener = TcpListener::bind("[::]:8080").await.unwrap();

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{
        Router,
        body::{Body, to_bytes},
        extract::State,
        http::{HeaderValue, Method, StatusCode},
        response::IntoResponse,
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
        // One-shot channel used by tests to inspect exactly one observed request.
        sender: Arc<Mutex<Option<oneshot::Sender<SeenRequest>>>>,
        // Stable upstream identifier returned in the x-upstream response header.
        id: &'static str,
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
            .header("x-upstream", state.id)
            .header("content-type", "text/plain")
            .body(Body::from("upstream ok"))
            .unwrap()
    }

    async fn spawn_upstream(
        // Human-readable upstream identity used by assertions.
        id: &'static str,
        // Channel used to report the captured request back to the test.
        sender: oneshot::Sender<SeenRequest>,
    ) -> (String, tokio::task::JoinHandle<()>) {
        let app = Router::new()
            .route("/{*path}", any(upstream_handler))
            .with_state(UpstreamState {
                sender: Arc::new(Mutex::new(Some(sender))),
                id,
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
        timeout: Duration,
    ) -> (String, tokio::task::JoinHandle<()>) {
        let app = build_app(routes, enable_auth, Env::Test, timeout);
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();
        let handle = tokio::spawn(async move {
            axum::serve(listener, app).await.unwrap();
        });

        (format!("http://{}", addr), handle)
    }

    fn route(prefix: &str, upstreams: &[&str]) -> Route {
        Route {
            // Prefix match key used by match_route.
            prefix: prefix.to_string(),
            // Convert borrowed test literals into owned String upstream URLs.
            upstreams: upstreams.iter().map(|s| s.to_string()).collect(),
        }
    }

    fn table() -> Vec<Route> {
        vec![
            route("/", &["http://root"]),
            route("/users", &["http://users-svc", "http://users-svc-2"]),
            route("/users/admin", &["http://admin-svc"]),
            route("/orders", &["http://orders-svc"]),
        ]
    }

    async fn spawn_slow_upstream() -> (String, tokio::task::JoinHandle<()>) {
        let app = Router::new().route("/{*path}", any(slow_upstream_handler));

        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();

        let handle = tokio::spawn(async move {
            axum::serve(listener, app).await.unwrap();
        });

        (format!("http://{}", addr), handle)
    }

    async fn slow_upstream_handler(req: Request<Body>) -> impl IntoResponse {
        let ms = req
            .headers()
            .get("x-test-sleep-ms")
            .and_then(|v| v.to_str().ok())
            .and_then(|v| v.parse::<u64>().ok())
            .unwrap_or(0);

        if ms > 0 {
            tracing::info!("injecting artificial delay: {}ms", ms);
            tokio::time::sleep(Duration::from_millis(ms)).await;
        }

        (axum::http::StatusCode::OK, "ok")
    }

    #[test]
    fn exact_match() {
        let routes = table();

        assert_eq!(
            match_route("/orders", &routes).and_then(|r| r.upstreams.first().map(|u| u.as_str())),
            Some("http://orders-svc")
        );
    }

    #[test]
    fn longer_prefix_wins_over_shorter() {
        // "/users/admin/1" matches both "/users" and "/users/admin"; the longer one must win.
        let routes = table();
        assert_eq!(
            match_route("/users/admin/1", &routes)
                .and_then(|r| r.upstreams.first().map(|u| u.as_str())),
            Some("http://admin-svc")
        );
    }

    #[test]
    fn shorter_prefix_used_when_no_longer_match() {
        // "/users/42" matches "/users" but not "/users/admin".
        let routes = table();
        assert_eq!(
            match_route("/users/42", &routes).and_then(|r| r.upstreams.first().map(|u| u.as_str())),
            Some("http://users-svc")
        );
    }

    #[test]
    fn root_catch_all() {
        // "/unknown" only matches "/".
        let routes = table();
        assert_eq!(
            match_route("/unknown", &routes).and_then(|r| r.upstreams.first().map(|u| u.as_str())),
            Some("http://root")
        );
    }

    #[test]
    fn no_match_returns_none() {
        let routes = vec![route("/api", &["http://api"])];

        assert!(match_route("/other", &routes).is_none());
    }

    #[test]
    fn partial_segment_does_not_match_longer_prefix() {
        // "/users/administrator" starts with "/users/admin" byte-for-byte, but "admin" is not
        // a complete segment — the next char is 'i', not '/'. Must fall back to "/users".
        let routes = table();
        assert_eq!(
            match_route("/users/administrator", &routes)
                .and_then(|r| r.upstreams.first().map(|u| u.as_str())),
            Some("http://users-svc")
        );
    }

    #[test]
    fn param_segment_matches_any_value() {
        let routes = vec![route("/users/:id", &["http://users-svc"])];

        assert_eq!(
            match_route("/users/42", &routes).and_then(|r| r.upstreams.first().map(|u| u.as_str())),
            Some("http://users-svc")
        );
        assert_eq!(
            match_route("/users/abc", &routes)
                .and_then(|r| r.upstreams.first().map(|u| u.as_str())),
            Some("http://users-svc")
        );
        assert!(match_route("/users", &routes).is_none());
    }

    #[test]
    fn static_beats_param_at_same_depth() {
        let routes = vec![
            route("/users/:id", &["http://users-svc"]),
            route("/users/admin", &["http://admin-svc"]),
        ];

        assert_eq!(
            match_route("/users/admin", &routes)
                .and_then(|r| r.upstreams.first().map(|u| u.as_str())),
            Some("http://admin-svc")
        );
        assert_eq!(
            match_route("/users/42", &routes).and_then(|r| r.upstreams.first().map(|u| u.as_str())),
            Some("http://users-svc")
        );
    }

    #[test]
    fn equal_specificity_keeps_first_configured_route() {
        let routes = vec![
            route("/users/:id", &["http://users-by-id"]),
            route("/users/:name", &["http://users-by-name"]),
        ];

        assert_eq!(
            match_route("/users/alice", &routes)
                .and_then(|r| r.upstreams.first().map(|u| u.as_str())),
            Some("http://users-by-id")
        );
    }

    #[test]
    fn trailing_slash_matches() {
        let routes = vec![route("/users", &["http://users"])];

        // /users/ same to /users
        assert_eq!(
            match_route("/users/", &routes).and_then(|r| r.upstreams.first().map(|u| u.as_str())),
            Some("http://users")
        );
    }

    #[test]
    fn root_matches_root() {
        let routes = vec![route("/", &["http://root"])];

        assert_eq!(
            match_route("/", &routes).and_then(|r| r.upstreams.first().map(|u| u.as_str())),
            Some("http://root")
        );
    }

    #[test]
    fn multiple_params_match() {
        let routes = vec![route("/users/:id/orders/:order_id", &["http://svc"])];

        assert!(match_route("/users/123/orders/456", &routes).is_some());
    }

    #[test]
    fn static_beats_param_deeply() {
        let routes = vec![
            route("/users/:id/orders", &["http://param"]),
            route("/users/admin/orders", &["http://admin"]),
        ];

        assert_eq!(
            match_route("/users/admin/orders", &routes)
                .and_then(|r| r.upstreams.first().map(|u| u.as_str())),
            Some("http://admin")
        );
    }

    #[tokio::test]
    async fn proxy_forwards_method_query_headers_body_and_response_headers() {
        let (tx, rx) = oneshot::channel();
        let (upstream_url, upstream_handle) = spawn_upstream("u1", tx).await;
        let (gateway_url, gateway_handle) = spawn_gateway(
            vec![route("/users", &[upstream_url.as_str()])],
            false,
            Duration::from_secs(15),
        )
        .await;

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
            Some(&HeaderValue::from_static("u1"))
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
        let (gateway_url, gateway_handle) = spawn_gateway(
            vec![route("/users", &["http://127.0.0.1:1"])],
            false,
            Duration::from_secs(15),
        )
        .await;

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

        let upstream = format!("http://{}", addr);

        let (gateway_url, gateway_handle) = spawn_gateway(
            vec![route("/users", &[upstream.as_str()])],
            false,
            Duration::from_secs(15),
        )
        .await;

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
        let (upstream_url, upstream_handle) = spawn_upstream("u1", tx).await;

        let (gateway_url, gateway_handle) = spawn_gateway(
            vec![route("/users", &[upstream_url.as_str()])],
            false,
            Duration::from_secs(15),
        )
        .await;

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

    #[tokio::test]
    async fn health_returns_200() {
        use tower::ServiceExt;

        let app = build_app(vec![], false, Env::Test, Duration::from_secs(15));

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/health")
                    .body(axum::body::Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn ready_returns_200() {
        use tower::ServiceExt;

        let app = build_app(vec![], false, Env::Test, Duration::from_secs(15));

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/ready")
                    .body(axum::body::Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn auth_returns_401_without_authorization_header() {
        let (gateway_url, gateway_handle) = spawn_gateway(
            vec![route("/users", &["http://127.0.0.1:1"])],
            true,
            Duration::from_secs(15),
        )
        .await;

        let response = reqwest::Client::new()
            .get(format!("{}/users/123", gateway_url))
            .send()
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);

        gateway_handle.abort();
    }

    #[tokio::test]
    async fn auth_allows_request_with_authorization_header() {
        let (tx, rx) = oneshot::channel();

        let (upstream_url, upstream_handle) = spawn_upstream("u1", tx).await;

        let (gateway_url, gateway_handle) = spawn_gateway(
            vec![route("/users", &[upstream_url.as_str()])],
            true,
            Duration::from_secs(15),
        )
        .await;

        let response = reqwest::Client::new()
            .get(format!("{}/users/123", gateway_url))
            .header("authorization", "Bearer test-token")
            .send()
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::CREATED);

        let seen = rx.await.unwrap();

        assert_eq!(seen.path_and_query, "/users/123");

        gateway_handle.abort();
        upstream_handle.abort();
    }

    #[tokio::test]
    async fn health_returns_ok_body() {
        use tower::ServiceExt;

        let app = build_app(vec![], false, Env::Test, Duration::from_secs(15));

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/health")
                    .body(axum::body::Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();

        assert_eq!(&body[..], b"OK");
    }

    #[tokio::test]
    async fn request_id_is_generated_for_upstream_request() {
        let (tx, rx) = oneshot::channel();

        let (upstream_url, upstream_handle) = spawn_upstream("u1", tx).await;

        let (gateway_url, gateway_handle) = spawn_gateway(
            vec![route("/users", &[upstream_url.as_str()])],
            false,
            Duration::from_secs(15),
        )
        .await;

        reqwest::Client::new()
            .get(format!("{}/users/123", gateway_url))
            .send()
            .await
            .unwrap();

        let seen = rx.await.unwrap();

        println!("{:#?}", seen.headers);

        assert!(seen.headers.get("x-request-id").is_some());

        gateway_handle.abort();
        upstream_handle.abort();
    }

    #[tokio::test]
    async fn request_id_is_generated_and_forwarded() {
        let (tx, rx) = oneshot::channel();

        let (upstream_url, upstream_handle) = spawn_upstream("u1", tx).await;

        let (gateway_url, gateway_handle) = spawn_gateway(
            vec![route("/users", &[upstream_url.as_str()])],
            false,
            Duration::from_secs(15),
        )
        .await;

        let response = reqwest::Client::new()
            .get(format!("{}/users/123", gateway_url))
            .send()
            .await
            .unwrap();

        assert!(response.headers().get("x-request-id").is_some());

        let seen = rx.await.unwrap();

        assert!(seen.headers.get("x-request-id").is_some());

        gateway_handle.abort();
        upstream_handle.abort();
    }

    #[tokio::test]
    async fn request_id_is_preserved_when_supplied() {
        let (tx, rx) = oneshot::channel();

        let (upstream_url, upstream_handle) = spawn_upstream("u1", tx).await;

        let (gateway_url, gateway_handle) = spawn_gateway(
            vec![route("/users", &[upstream_url.as_str()])],
            false,
            Duration::from_secs(15),
        )
        .await;

        let response = reqwest::Client::new()
            .get(format!("{}/users/123", gateway_url))
            .header("x-request-id", "my-request-id")
            .send()
            .await
            .unwrap();

        assert_eq!(
            response
                .headers()
                .get("x-request-id")
                .unwrap()
                .to_str()
                .unwrap(),
            "my-request-id"
        );

        let seen = rx.await.unwrap();

        assert_eq!(
            seen.headers.get("x-request-id").unwrap().to_str().unwrap(),
            "my-request-id"
        );

        gateway_handle.abort();
        upstream_handle.abort();
    }

    #[test]
    fn test_filter_headers_removes_all_hop_by_hop_headers() {
        let mut headers = HeaderMap::new();

        headers.insert("connection", "x".parse().unwrap());
        headers.insert("keep-alive", "x".parse().unwrap());
        headers.insert("proxy-authenticate", "x".parse().unwrap());
        headers.insert("proxy-authorization", "x".parse().unwrap());
        headers.insert("te", "x".parse().unwrap());
        headers.insert("trailer", "x".parse().unwrap());
        headers.insert("transfer-encoding", "x".parse().unwrap());
        headers.insert("upgrade", "x".parse().unwrap());

        headers.insert("x-ok", "1".parse().unwrap());

        let filtered = HopByHopFilter.filter(&headers);

        assert!(filtered.get("connection").is_none());
        assert!(filtered.get("keep-alive").is_none());
        assert!(filtered.get("proxy-authenticate").is_none());
        assert!(filtered.get("proxy-authorization").is_none());
        assert!(filtered.get("te").is_none());
        assert!(filtered.get("trailer").is_none());
        assert!(filtered.get("transfer-encoding").is_none());
        assert!(filtered.get("upgrade").is_none());

        assert!(filtered.get("x-ok").is_some());
    }

    #[tokio::test]
    async fn gateway_timeout_via_header_injection() {
        use axum::body::Body;
        use axum::http::Request;
        use std::time::Duration;
        use tower::ServiceExt;

        // 1. start controlled upstream
        let (upstream_url, upstream_handle) = spawn_slow_upstream().await;

        // 2. build gateway with timeout smaller than injected delay
        const DELAY: u64 = 100;
        let app = build_app(
            vec![route("/users", &[upstream_url.as_str()])],
            false,
            Env::Test,
            Duration::from_millis(DELAY - 1),
        );

        // 3. call gateway WITH trigger header
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/users/123")
                    .header("x-test-sleep-ms", DELAY.to_string()) // 👈 triggers upstream delay
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        // 4. assert timeout happened
        assert_eq!(response.status(), StatusCode::GATEWAY_TIMEOUT);

        upstream_handle.abort();
    }

    #[tokio::test]
    async fn round_robin_distributes_requests() {
        // Separate observation channels so each upstream can report one request.
        let (tx1, rx1) = oneshot::channel();
        let (tx2, rx2) = oneshot::channel();

        // Distinct IDs let us assert exact upstream selection order.
        const STREAM_ID_1: &str = "upstream-1";
        const STREAM_ID_2: &str = "upstream-2";
        let (u1, h1) = spawn_upstream(STREAM_ID_1, tx1).await;
        let (u2, h2) = spawn_upstream(STREAM_ID_2, tx2).await;

        // Route /users traffic to both upstreams to exercise round-robin selection.
        let (gateway_url, gateway_handle) = spawn_gateway(
            vec![route("/users", &[u1.as_str(), u2.as_str()])],
            false,
            Duration::from_secs(15),
        )
        .await;

        // First request should hit upstream-1.
        let response1 = reqwest::get(format!("{}/users/1", gateway_url))
            .await
            .unwrap();
        // Second request should hit upstream-2.
        let response2 = reqwest::get(format!("{}/users/2", gateway_url))
            .await
            .unwrap();

        // Assert upstream identity from response headers.
        assert_eq!(response1.headers().get("x-upstream").unwrap(), STREAM_ID_1);
        assert_eq!(response2.headers().get("x-upstream").unwrap(), STREAM_ID_2);

        // Pull captured requests from each upstream observer.
        let seen1 = rx1.await.unwrap();
        let seen2 = rx2.await.unwrap();

        // Ensure each upstream observed the expected URL path.
        assert_eq!(seen1.path_and_query, "/users/1");
        assert_eq!(seen2.path_and_query, "/users/2");

        gateway_handle.abort();
        h1.abort();
        h2.abort();
    }
}
