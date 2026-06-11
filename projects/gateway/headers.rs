use axum::http::{HeaderMap, HeaderName};

pub struct HopByHopFilter;

impl HopByHopFilter {
    // HTTP allows repeated headers
    // Set-Cookie: a=1
    // Set-Cookie: b=2
    // Internally optimized storage:
    // ("Set-Cookie", "a=1")
    // (None, "b=2")
    // Meaning:
    // “same header as previous entry”
    // So Rust uses:
    // Option<HeaderName> if destructure name from headers and
    // it is not match insert signature for key
    // HeaderMap implements FromIterator
    pub fn filter(&self, headers: &HeaderMap) -> HeaderMap {
        headers
            .iter()
            .filter(|(name, _)| !is_hop_by_hop(name))
            .map(|(name, value)| (name.clone(), value.clone()))
            .collect()
    }
}

// RFC 9110 says these hop-by-hop headers must NOT be forwarded
fn is_hop_by_hop(name: &HeaderName) -> bool {
    matches!(
        name.as_str().to_ascii_lowercase().as_str(),
        "connection"
            | "keep-alive"
            | "proxy-authenticate"
            | "proxy-authorization"
            | "te"
            | "trailer"
            | "transfer-encoding"
            | "upgrade"
    )
}
