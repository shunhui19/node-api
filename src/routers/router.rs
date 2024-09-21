use axum::{routing::get, Router};

pub fn get_routers() -> Router {
    Router::new().route("/", get(root))
}

async fn root() -> String {
    "Hello".to_string()
}
