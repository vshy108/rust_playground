use std::sync::{
    Arc,
    atomic::{AtomicUsize, Ordering},
};

#[derive(Clone)]
pub struct LoadBalancer {
    // Atomic counter is shared so each request gets the next upstream index.
    counter: Arc<AtomicUsize>,
}

// The counter is shared so cloned states continue the same round-robin sequence.
impl LoadBalancer {
    // Starts selection from index 0.
    pub fn new() -> Self {
        Self {
            counter: Arc::new(AtomicUsize::new(0)),
        }
    }

    // Picks one upstream and returns it as owned data for async-safe use.
    pub fn pick(&self, upstreams: &[String]) -> Option<String> {
        // No upstream configured for this route.
        if upstreams.is_empty() {
            return None;
        }

        // Each request advances the shared counter and wraps across the upstream list.
        // Relaxed ordering is sufficient because we only need unique progression, not synchronization.
        let idx = self.counter.fetch_add(1, Ordering::Relaxed);

        // Wrap around when idx grows beyond the upstream list length.
        let selected = &upstreams[idx % upstreams.len()];
        // Return an owned URL so callers can move it across async boundaries safely.
        Some(selected.clone())
    }
}
