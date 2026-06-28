use std::sync::Arc;

use crate::gateway::{round_robin::LoadBalancer, types::Route};

#[derive(Clone)]
pub struct AppState {
    // Route table used by the matcher to choose an upstream group.
    pub routes: Vec<Route>,
    // Shared HTTP client (connection pool, timeouts, keep-alive).
    pub client: reqwest::Client,
    // Shared load balancer state so round-robin order survives across requests.
    pub lb: Arc<LoadBalancer>,
}
