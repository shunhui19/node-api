use axum::{
    body::Body,
    http::{Request, StatusCode},
    middleware::Next,
    response::Response,
};
use std::{
    collections::HashMap,
    sync::{atomic::AtomicUsize, atomic::Ordering, Arc, Mutex},
};
use tracing::warn;

use crate::CONFIG;

use super::jwt::validate_jwt;

#[derive(Clone)]
pub struct RateLimiter {
    config: Arc<Mutex<RateLimitConfig>>,
    current_request_num: Arc<Mutex<HashMap<String, AtomicUsize>>>,
}

// every key has a config.
#[derive(Clone)]
pub struct RateLimitConfig {
    limits: HashMap<String, RateLimitInfo>,
}

impl Default for RateLimitConfig {
    fn default() -> Self {
        let mut limit_hash_map = HashMap::new();
        limit_hash_map.insert(
            CONFIG.token.secret.clone(),
            RateLimitInfo {
                // qps: 1,
                total_request_num: 15,
            },
        );

        Self {
            limits: limit_hash_map,
        }
    }
}

// Default config for every key.
#[derive(Clone)]
struct RateLimitInfo {
    // qps: usize,
    total_request_num: usize,
}

impl RateLimiter {
    pub fn new(config: RateLimitConfig) -> Self {
        Self {
            config: Arc::new(Mutex::new(config)),
            current_request_num: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    // fn update_config(&self, qps: usize, total_request_num: usize) {
    //     todo!("todo");
    // }

    fn check_and_increment(&self, key: &str) -> bool {
        let mut current_request_num = match self.current_request_num.lock() {
            Ok(lock) => lock,
            Err(err) => {
                warn!("Failed to acquire lock on current_request_num: {:?}", err);
                return false;
            }
        };
        let total_request_num = current_request_num
            .entry(key.to_string())
            .or_insert_with(|| AtomicUsize::new(0));

        let config = match self.config.lock() {
            Ok(lock) => lock,
            Err(err) => {
                warn!("Failed to acquire lock on config: {:?}", err);
                return false;
            }
        };

        // The maximum nubmer of requests was exceeded.
        if total_request_num.load(Ordering::SeqCst)
            >= config
                .limits
                .get(key)
                .map(|limit| limit.total_request_num)
                .unwrap_or(usize::MAX)
        {
            return false;
        }

        total_request_num.fetch_add(1, Ordering::SeqCst);
        true
    }

    // fn get_qps_for_key(&self, key: &str) -> usize {
    //     match self.config.lock() {
    //         Ok(lock) => lock
    //             .limits
    //             .get(key)
    //             .map(|limit| limit.qps)
    //             .unwrap_or(usize::MAX),
    //         Err(err) => {
    //             warn!("Failed to acquire lock on config: {:?}", err);
    //             usize::MAX
    //         }
    //     }
    // }

    // pub fn build_layer_for_key(&self, key: &str) -> impl Layer<Body> + Clone {
    //     let qps = self.get_qps_for_key(key);
    //     RateLimitLayer::new(qps as u64, Duration::from_secs(1))
    // }
}

pub async fn total_request_limit(
    req: Request<Body>,
    rate_limiter: RateLimiter,
    next: Next,
) -> Result<Response, StatusCode> {
    let token = match req.headers().get("Authorization") {
        Some(token) => token,
        None => {
            warn!("Authorization header is missing");
            return Err(StatusCode::UNAUTHORIZED);
        }
    };

    let token_str = match token.to_str() {
        Ok(key) => key,
        Err(_) => {
            warn!("Invalid token format, token: {:?}", token);
            return Err(StatusCode::BAD_REQUEST);
        }
    };

    let key = match validate_jwt(token_str) {
        Ok(key) => key,
        Err(_) => {
            warn!("Invali token or JWT validation failed");
            return Err(StatusCode::UNAUTHORIZED);
        }
    };

    if !rate_limiter.check_and_increment(key.sub.as_str()) {
        warn!("the requests of the key({:?}) is exceeded", key);
        return Err(StatusCode::TOO_MANY_REQUESTS);
    }

    Ok(next.run(req).await)
}
