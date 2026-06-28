use axum::response::IntoResponse;
use http::StatusCode;

pub async fn health() -> impl IntoResponse {
    "OK"
}

pub async fn ready() -> StatusCode {
    StatusCode::OK
}
