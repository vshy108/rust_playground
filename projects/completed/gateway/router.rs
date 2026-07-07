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
        // -----------------------------
        // System health endpoints
        // -----------------------------
        // Basic liveness endpoint.
        .route("/health", get(health))
        // Readiness endpoint for orchestrators.
        .route("/ready", get(ready))
        // -----------------------------
        // Catch-all gateway proxy route
        // -----------------------------
        // Matches ALL paths not explicitly defined above.
        // Example:
        //   /users
        //   /orders/123
        //   /anything/else
        //
        // IMPORTANT:
        // - This must be last because Axum evaluates routes top-down.
        // - Wildcard captures full remaining path.
        // All unmatched traffic enters proxy_handler.
        .route("/{*path}", any(proxy_handler))
        // -----------------------------
        // Shared application state
        // -----------------------------
        // Injects AppState into all handlers via axum extractor:
        //   State<AppState>
        //
        // Avoids global variables:
        // - improves testability
        // - enables dependency swapping (mock clients, test routes)
        // - isolates runtime configuration
        // All handlers read the same cloned AppState instance.
        .with_state(state)
}
