use std::sync::Arc;

use axum::middleware::Next;
use axum::{
    body::Body,
    http::{Request, StatusCode},
    response::Response,
};
use chrono::{Duration, Utc};
use jsonwebtoken::{
    decode, encode, errors::Result as JwtResult, Algorithm, DecodingKey, EncodingKey, Header,
    Validation,
};
use serde::{Deserialize, Serialize};
use tracing::warn;

pub const SECRET: &str = "hell, rust";

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    // user unique identify
    pub sub: String,
    // expire time
    exp: usize,
}

pub fn validate_jwt(token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    let token_data = decode(
        token,
        &DecodingKey::from_secret(SECRET.as_ref()),
        &Validation::new(Algorithm::HS256),
    )?;
    Ok(token_data.claims)
}

fn generate_jwt(user_id: &str) -> JwtResult<String> {
    // TODO: how long expired is get from config file or DB
    let claims = Claims {
        sub: user_id.to_owned(),
        exp: (Utc::now() + Duration::hours(6)).timestamp() as usize,
    };

    let token = encode(
        &Header::new(Algorithm::HS256),
        &claims,
        &EncodingKey::from_secret(SECRET.as_bytes()),
    )?;
    Ok(token)
}

pub async fn get_jwt() -> Result<String, StatusCode> {
    match generate_jwt(SECRET) {
        Ok(token) => Ok(token),
        Err(_) => {
            warn!("Generate token failed");
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn jwt_middleware(mut req: Request<Body>, next: Next) -> Result<Response, StatusCode> {
    if let Some(auth_header) = req.headers().get("Authorization") {
        if let Ok(token) = auth_header.to_str() {
            match validate_jwt(token) {
                Ok(claims) => {
                    req.extensions_mut().insert(Arc::new(claims));
                    return Ok(next.run(req).await);
                }
                Err(e) => {
                    warn!("Authorization jwt failed: {:?}", e);
                    return Err(StatusCode::UNAUTHORIZED);
                }
            }
        }
    }
    warn!("Authorization jwt failed");
    Err(StatusCode::UNAUTHORIZED)
}
