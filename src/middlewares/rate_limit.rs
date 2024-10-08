use axum::{
    body::Body,
    http::{Request, StatusCode},
    response::IntoResponse,
};
use std::time::Duration;
use std::{
    collections::HashMap,
    sync::{atomic::AtomicUsize, atomic::Ordering, Arc, Mutex},
};
use tower::{limit::RateLimitLayer, Layer};
use tracing::{debug, warn};

use super::jwt::{validate_jwt, SECRET};

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
            SECRET.to_string(),
            RateLimitInfo {
                qps: 1,
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
    qps: usize,
    total_request_num: usize,
}

impl RateLimiter {
    pub fn new(config: RateLimitConfig) -> Self {
        Self {
            config: Arc::new(Mutex::new(config)),
            current_request_num: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    fn update_config(&self, qps: usize, total_request_num: usize) {
        todo!("todo");
    }

    fn check_and_increment(&self, key: &str) -> bool {
        debug!("key is: {:?}", key);
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

    fn get_qps_for_key(&self, key: &str) -> usize {
        match self.config.lock() {
            Ok(lock) => lock
                .limits
                .get(key)
                .map(|limit| limit.qps)
                .unwrap_or(usize::MAX),
            Err(err) => {
                warn!("Failed to acquire lock on config: {:?}", err);
                usize::MAX
            }
        }
    }

    pub fn build_layer_for_key(&self, key: &str) -> impl Layer<Body> + Clone {
        let qps = self.get_qps_for_key(key);
        RateLimitLayer::new(qps as u64, Duration::from_secs(1))
    }
}

pub async fn rate_limit(req: Request<Body>, rate_limiter: RateLimiter) -> impl IntoResponse {
    // TODO: optimization unwrap().
    let token = req
        .headers()
        .get("Authorization")
        .unwrap()
        .to_str()
        .unwrap();
    let key = validate_jwt(token).unwrap();

    if !rate_limiter.check_and_increment(key.sub.as_str()) {
        return (StatusCode::TOO_MANY_REQUESTS, "Rate limit exceeded");
    }

    (StatusCode::OK, "Request processed")
}
