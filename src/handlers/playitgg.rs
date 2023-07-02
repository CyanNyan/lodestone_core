use axum::{routing::{post, get}, Router};
use crate::playitgg::{start_tunnel, kill_tunnel, generate_signup_link, confirm_singup};

use crate::AppState;

pub fn get_playitgg_routes(state: AppState) -> Router {
    Router::new()
        .route("/playitgg/run_tunnel", post(start_tunnel))
        .route("/playitgg/kill_tunnel", post(kill_tunnel))
        .route("/playitgg/generate_signup_link", get(generate_signup_link))
        .route("/playitgg/confirm_signup", get(confirm_singup))
        .with_state(state)
}