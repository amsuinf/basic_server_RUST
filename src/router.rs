use crate::handler::hello_world_handler;
use axum::routing::{Router, get};

pub fn router() -> Router {
    Router::new().route("/", get(hello_world_handler))
}
