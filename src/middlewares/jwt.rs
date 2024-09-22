use chrono::{Duration, Utc};
use jsonwebtoken::{
    decode, encode, errors::Result as JwtResult, Algorithm, DecodingKey, EncodingKey, Header,
    Validation,
};
use serde::{Deserialize, Serialize};

const SECRET: &str = "hell, rust";

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    sub: String,
    exp: usize,
}

pub fn validate_jwt(token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    let token_data = decode(
        token,
        &DecodingKey::from_secret("secret".as_ref()),
        &Validation::new(Algorithm::HS256),
    )?;
    Ok(token_data.claims)
}

pub fn generate_jwt(user_id: &str) -> JwtResult<String> {
    let claims = Claims {
        sub: user_id.to_owned(),
        exp: (Utc::now() + Duration::hours(1)).timestamp() as usize,
    };

    let token = encode(
        &Header::new(Algorithm::HS256),
        &claims,
        &EncodingKey::from_secret(SECRET.as_bytes().as_ref()),
    )?;
    Ok(token)
}
