use axum::{
    Router,
    routing::{any, get},
};

use crate::gateway::{
    handlers::{health, ready},
    proxy_handler,
    state::AppState,
};

pub fn build_router(state: AppState) -> Router {
    Router::new()
        .route("/health", get(health))
        .route("/ready", get(ready))
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
        .with_state(state)
}
