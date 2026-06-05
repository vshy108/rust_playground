// example must have main function
// Goal: Architecture

// Build:

// ```bash
// cargo run --bin api_gateway
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
// - [ ] Basic API Gateway
// - [ ] Prefix-based routing
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

use axum::{Router, extract::State, http::Request, routing::any};
use tokio::net::TcpListener;

#[allow(dead_code)]
#[derive(Clone, Debug)]
struct Route {
    prefix: String,
    upstream: String,
}

#[derive(Clone)]
struct AppState {
    routes: Vec<Route>,
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

async fn proxy_handler(State(state): State<AppState>, req: Request<axum::body::Body>) {
    let path = req.uri().path();

    let route = match_route(path, &state.routes);

    println!("matched route: {:?}", route);
}

fn build_app(routes: Vec<Route>) -> Router {
    Router::new()
        // gateway already has its own router hence use single catch-all route, /*path
        // Path segments must not start with `*`. For wildcard capture, use `{*wildcard}`.
        // If you meant to literally match a segment starting with an asterisk,
        // call `without_v07_checks` on the router.
        .route("/{*path}", any(proxy_handler))
        // with_state here will pass to route handler, proxy_handler 1st argument State
        // not using global variables, harder to test, hidden dependencies, difficult
        // to swap configurations
        // not capture routes in a closure,
        // .route("/*path", any(move |req| async move {, messy if State larger
        .with_state(AppState { routes })
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

    let app = build_app(routes);

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
    use axum::{body::Body, http::StatusCode};
    use tower::ServiceExt;

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
    async fn users_route_is_selected() {
        let app = build_app(vec![route("/users", "http://users")]);

        let response = app
            // Consume this Service, calling it with the provided request once it is ready.
            .oneshot(
                Request::builder()
                    .uri("/users/123")
                    .body(Body::empty())
                    // invalid uri, header and method can make the request builder failed
                    // hence need unwrap it from Result<>
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }
}
