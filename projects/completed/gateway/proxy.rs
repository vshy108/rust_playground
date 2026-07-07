use axum::{
    body::Body,
    http::{Response, StatusCode},
};

use crate::gateway::headers::HopByHopFilter;
use crate::{AppState, Route};

pub async fn forward_request(
    state: &AppState,
    route: &Route,
    parts: http::request::Parts, // request head after splitting the body stream
    body: Body,
) -> Result<Response<Body>, StatusCode> {
    // Preserve original HTTP method (GET/POST/PUT/...).
    let method = parts.method;
    // Reusable header sanitization helper.
    let filter = HopByHopFilter;

    // -----------------------------
    // 1. Extract request target path
    // -----------------------------
    let uri = parts.uri;

    // NOTE:
    // path_and_query() preserves query string (?a=b)
    // fallback ensures /path-only requests still work
    let path_and_query = uri
        .path_and_query()
        .map(|pq| pq.as_str())
        .unwrap_or_else(|| uri.path());

    // -----------------------------
    // 2. Load balancer selects upstream instance
    // -----------------------------
    let upstream = state
        .lb
        .pick(&route.upstreams)
        .ok_or(StatusCode::BAD_GATEWAY)?;

    // -----------------------------
    // 3. Construct final upstream URL safely
    // -----------------------------
    // Avoid:
    //   - double slashes (//)
    //   - missing slash between host and path
    let url = format!(
        "{}/{}",
        upstream.trim_end_matches('/'),
        path_and_query.trim_start_matches('/')
    );

    tracing::debug!(%url, "proxy upstream target");

    // -----------------------------
    // 4. Stream request body (no buffering)
    // -----------------------------
    let stream = body.into_data_stream();
    // NOTE: reqwest features need to include stream
    let reqwest_body = reqwest::Body::wrap_stream(stream);

    // -----------------------------
    // 5. Filter hop-by-hop request headers
    // -----------------------------
    let forwarded_headers = filter.filter(&parts.headers);

    // -----------------------------
    // 6. Execute upstream request
    // -----------------------------
    let resp = state
        .client
        .request(method, &url)
        .headers(forwarded_headers)
        .body(reqwest_body)
        .send()
        .await
        .map_err(|_| StatusCode::BAD_GATEWAY)?;

    // Capture status and headers before consuming response body stream.
    let status = resp.status();
    let resp_headers = resp.headers().clone();

    // -----------------------------
    // 7. Stream response body (zero-copy)
    // -----------------------------
    let body = Body::from_stream(resp.bytes_stream());

    // -----------------------------
    // 8. Filter hop-by-hop response headers
    // -----------------------------
    // Response headers:
    // Content-Type
    // Content-Length
    // Cache-Control
    // ETag
    // Location
    // Ensures headers are inserted at build-time
    // Avoids unsafe mutation after body creation
    // Prevents silent header drop bugs
    let filtered_headers = filter.filter(&resp_headers);

    // -----------------------------
    // 9. Build final response
    // -----------------------------
    let mut response = Response::builder()
        .status(status)
        .body(body)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Insert headers safely (preserve axum internals)
    let headers_mut = response.headers_mut();

    for (k, v) in filtered_headers.iter() {
        // Re-attach allowed upstream response headers.
        headers_mut.insert(k, v.clone());
    }

    // -----------------------------
    // 10. Request ID propagation (debug trace continuity)
    // -----------------------------
    if let Some(id) = parts.headers.get("x-request-id") {
        // Preserve correlation ID for end-to-end tracing.
        headers_mut.insert("x-request-id", id.clone());
    }

    Ok(response)
}
