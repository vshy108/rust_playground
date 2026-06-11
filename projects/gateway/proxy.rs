use axum::{
    body::Body,
    http::{Response, StatusCode},
};

use crate::gateway::headers::HopByHopFilter;
use crate::{AppState, Route};

pub async fn forward_request(
    state: &AppState,
    route: &Route,
    parts: http::request::Parts, // cargo add http
    body: Body,
) -> Result<Response<Body>, StatusCode> {
    let method = parts.method;
    let filter = HopByHopFilter;
    let forwarded_headers = filter.filter(&parts.headers);

    let uri = parts.uri;
    // let path = req.uri().path();
    let path = uri.path();

    let stream = body.into_data_stream();
    // NOTE: reqwest features need to include stream
    let reqwest_body = reqwest::Body::wrap_stream(stream);
    // NOTE: path_and_query() no need query feature, but query() needs
    let path_and_query = uri.path_and_query().map(|pq| pq.as_str()).unwrap_or(path);

    // let url = format!("{}{}", route.upstream, path);
    // query() expects something serializable, such as: .query(&[("foo", "bar")])
    let url = format!("{}{}", route.upstream, path_and_query);
    println!("Redirect path and query: {}", url);

    // forwards only method and URL but not request body, query string, headers
    let resp = state
        .client
        .request(method, url)
        .headers(forwarded_headers)
        .body(reqwest_body)
        .send()
        .await
        .map_err(|_| StatusCode::BAD_GATEWAY)?;

    let resp_status = resp.status();
    let resp_headers = resp.headers().clone();

    // resp.bytes_stream()
    // resp.bytes(self) but not bytes(&self) hence it will consume the resp
    // let body = resp.bytes().await.map_err(|_| StatusCode::BAD_GATEWAY)?;
    let resp_stream = resp.bytes_stream();
    // Upstream → chunk → gateway → chunk → client
    // no more full body in RAM
    let resp_body = axum::body::Body::from_stream(resp_stream);

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

    let mut builder = Response::builder().status(resp_status);

    // extend headers_mut is somehow unsafe
    let headers = builder.headers_mut().unwrap();
    for (k, v) in filtered_headers.iter() {
        headers.insert(k, v.clone());
    }

    // NOTE: proxy server not pass back x-request-id in response
    if let Some(id) = parts.headers.get("x-request-id") {
        headers.insert("x-request-id", id.clone());
    }

    let response = builder
        .body(resp_body)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(response)
}
