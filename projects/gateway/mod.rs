pub mod handlers;
pub mod headers;
pub mod proxy;
pub mod route_matcher;
pub mod router;
pub mod state;
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
    let (parts, body) = req.into_parts();

    let route = match_route(parts.uri.path(), &state.routes).ok_or(StatusCode::NOT_FOUND)?;

    forward_request(&state, route, parts, body).await
}
