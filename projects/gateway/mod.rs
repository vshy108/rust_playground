// Lightweight liveness/readiness handlers.
pub mod handlers;
// Hop-by-hop header filtering utilities.
pub mod headers;
// Upstream forwarding implementation.
pub mod proxy;
// Round-robin upstream picker.
pub mod round_robin;
// Prefix and parameter route matching logic.
pub mod route_matcher;
// Public axum router construction.
pub mod router;
// Shared application state passed via axum extractors.
pub mod state;
// Core gateway domain types.
pub mod types;

use axum::{
    extract::State,
    http::{Request, StatusCode},
    response::Response,
};

use proxy::forward_request;
use route_matcher::match_route;
use state::AppState;

pub async fn proxy_handler(
    State(state): State<AppState>,
    req: Request<axum::body::Body>,
) -> Result<Response, StatusCode> {
    // Split request into immutable head + streaming body for forwarding.
    let (parts, body) = req.into_parts();

    // Resolve the best configured route for the incoming path.
    let route = match_route(parts.uri.path(), &state.routes).ok_or(StatusCode::NOT_FOUND)?;

    // Delegate transport details to the proxy module.
    forward_request(&state, route, parts, body).await
}
