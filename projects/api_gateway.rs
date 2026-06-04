// example must have main function
// Goal: Architecture

// Build:

// ```bash
// cargo run --bin api_gateway
// ```

// Learn:

// - middleware — tower `Layer`/`Service` traits; each layer wraps the inner service and can
//   intercept requests and responses (logging, auth, rate limiting) without changing handlers
// - resilience — retry on transient errors with exponential back-off; timeout cancels a slow
//   upstream; rate limiting caps requests per time window per client

// - API Gateway pattern — a single entry point that routes requests to internal services.
//   Clients only know the gateway; service locations remain hidden behind it.
//
// - Route matching — gateway selects an upstream service based on URL prefixes.
//   More specific routes should win over generic routes.
//
// - Route specificity — deeper paths are preferred over shallower paths.
//   Example:
//     "/users/admin" > "/users"
//   because it provides a more precise match.
//
// - Parameterized routes — segments beginning with ':' match any value.
//   Example:
//     "/users/:id"
//     "/users/42"
//     "/users/alice"
//
// - Catch-all routes — "/" acts as the lowest-priority fallback route.
//
// Progress:
//
// - [x] Route matching
// - [x] Route specificity resolution
// - [x] Parameterized route matching
//
// Extra:
//
// - [x] Basic API Gateway
// - [x] Prefix-based routing
// - [x] Request forwarding
// - [x] Header filtering (hop-by-hop header stripping)
// - [x] Upstream error handling (502 Bad Gateway)
// - [x] Integration tests (echo server, proxy forwarding)
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

#[derive(Clone, Debug)]
struct Route {
    prefix: String,
    upstream: String,
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
    // The returned route is borrowed from `routes`, not `path`, so the lifetime is named on
    // the route slice and the returned reference is tied to it.
    let mut best: Option<((usize, usize), &Route)> = None;

    for route in routes {
        let prefix_segs: Vec<&str> = route.prefix.split('/').filter(|s| !s.is_empty()).collect();
        let path_segs: Vec<&str> = path.split('/').filter(|s| !s.is_empty()).collect();

        if path_segs.len() < prefix_segs.len() {
            continue;
        }

        let matched = prefix_segs.iter().zip(path_segs.iter()).all(|(p, s)| {
            p.starts_with(':') || p == s
        });
        if !matched {
            continue;
        }

        let static_count = prefix_segs.iter().filter(|s| !s.starts_with(':')).count();
        let score = (prefix_segs.len(), static_count);

        match best {
            Some((best_score, _)) if best_score >= score => {}
            _ => best = Some((score, route)),
        }
    }

    best.map(|(_, route)| route)
}

fn main() {
    println!("api_gateway: route matching demo");
}

#[cfg(test)]
mod tests {
    use super::*;

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
}
