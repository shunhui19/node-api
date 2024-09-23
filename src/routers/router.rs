use axum::{routing::get, Router};

use crate::controllers::btc::btc_handle;
use crate::middlewares::jwt::get_jwt;

pub fn get_routers() -> Router {
    Router::new()
        .route("/", get(root))
        .route("/v1/btc", get(btc_handle))
        .route("/v1/auth/get_jwt", get(get_jwt))
}

async fn root() -> String {
    "Hello".to_string()
}
