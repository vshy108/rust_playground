pub mod headers;
pub mod proxy;
pub mod router;

use axum::{
    extract::State,
    http::{Request, StatusCode},
    response::Response,
};

use crate::AppState;
use proxy::forward_request;
use router::match_route;

pub async fn proxy_handler(
    State(state): State<AppState>,
    req: Request<axum::body::Body>,
) -> Result<Response, StatusCode> {
    let (parts, body) = req.into_parts();

    let route = match_route(parts.uri.path(), &state.routes).ok_or(StatusCode::NOT_FOUND)?;

    forward_request(&state, route, parts, body).await
}
