use axum::middleware;
use axum::{routing::get, Router};

use crate::controllers::btc::btc_handle;
use crate::middlewares::jwt::{get_jwt, jwt_middleware};
use crate::middlewares::log::log_middleware;

pub fn get_routers() -> Router {
    Router::new()
        .route("/", get(root))
        .route("/v1/btc", get(btc_handle).post(btc_handle))
        .layer(middleware::from_fn(jwt_middleware))
        .route("/v1/auth/get_jwt", get(get_jwt))
        .layer(middleware::from_fn(log_middleware))
}

async fn root() -> String {
    "Hello".to_string()
}
