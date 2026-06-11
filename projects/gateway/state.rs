use crate::gateway::types::Route;

#[derive(Clone)]
pub struct AppState {
    pub routes: Vec<Route>,
    pub client: reqwest::Client,
}
