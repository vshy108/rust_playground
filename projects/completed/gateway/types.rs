#[derive(Clone, Debug)]
pub struct Route {
    // Path prefix matched against the inbound request URI.
    pub prefix: String,
    // Multiple upstreams enable round-robin today and leave room for other strategies later.
    pub upstreams: Vec<String>,
}

#[allow(unused)]
#[derive(Clone, Copy)]
pub enum Env {
    // Test mode: deterministic behavior and looser middleware controls.
    Test,
    // Production mode: full operational middleware enabled.
    Prod,
}
