#[derive(Clone, Debug)]
pub struct Route {
    pub prefix: String,
    pub upstream: String,
}

#[allow(unused)]
#[derive(Clone, Copy)]
pub enum Env {
    Test,
    Prod,
}
