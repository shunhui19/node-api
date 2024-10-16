use axum::middleware;
use axum::{routing::get, Router};
use std::time::Duration;
use tower::ServiceBuilder;
use tower_http::{timeout::TimeoutLayer, trace::TraceLayer};

use crate::controllers::btc::btc_handle;
use crate::middlewares::jwt::{get_jwt, jwt_middleware};
use crate::middlewares::log::log_middleware;
use crate::middlewares::rate_limit::{total_request_limit, RateLimitConfig, RateLimiter};
use crate::CONFIG;

pub fn get_routers() -> Router {
    let rate_limiter = RateLimiter::new(RateLimitConfig::default());

    Router::new()
        .route("/", get(root))
        .route("/v1/btc", get(btc_handle).post(btc_handle))
        .layer(middleware::from_fn(jwt_middleware))
        .layer(middleware::from_fn(log_middleware))
        .layer(middleware::from_fn(move |req, next| {
            let rate_limiter = rate_limiter.clone();
            total_request_limit(req, rate_limiter, next)
        }))
        .route("/v1/auth/get_jwt", get(get_jwt))
        .layer(ServiceBuilder::new().layer(TraceLayer::new_for_http()))
        .layer(
            ServiceBuilder::new().layer(TimeoutLayer::new(Duration::from_secs(
                CONFIG.server.timeout as u64,
            ))),
        )
}

async fn root() -> String {
    "Hello".to_string()
}
