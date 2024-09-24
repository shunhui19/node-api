use axum::{
    body::{to_bytes, Body, Bytes},
    extract::Request,
    middleware::Next,
    response::Response,
};
use hyper::Method;
use serde_json::Value;

use tracing::{info, warn};

pub async fn log_middleware(req: Request<Body>, next: Next) -> Response {
    let method = req.method().clone();
    let uri = req.uri().to_string();
    let params = req.uri().query().unwrap_or("");

    match method {
        Method::POST => {
            let (parts, body) = req.into_parts();

            let body_bytes = match to_bytes(body, usize::MAX).await {
                Ok(bytes) => bytes,
                Err(e) => {
                    warn!("Failed to read request body: {}", e);
                    Bytes::new()
                }
            };

            let params: Value = serde_json::from_slice(&body_bytes).unwrap_or(Value::Null);

            info!(
                "Incoming request, method = {} params = {}, url = {}",
                method, params, uri
            );

            // rebuild request for next step request.
            let req = Request::from_parts(parts, Body::from(body_bytes));
            next.run(req).await
        }
        _ => {
            info!(
                "Incoming request, method = {} params = {}, url = {}",
                method, params, uri
            );
            next.run(req).await
        }
    }
}
