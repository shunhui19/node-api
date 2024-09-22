use axum::{routing::get, Router};

use crate::controllers::btc::btc_handle;

pub fn get_routers() -> Router {
    Router::new()
        .route("/", get(root))
        .route("/btc", get(btc_handle))
}

async fn root() -> String {
    "Hello".to_string()
}
