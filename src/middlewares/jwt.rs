use chrono::{Duration, Utc};
use jsonwebtoken::{
    decode, encode, errors::Result as JwtResult, Algorithm, DecodingKey, EncodingKey, Header,
    Validation,
};
use serde::{Deserialize, Serialize};

const SECRET: &str = "hell, rust";

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    // user unique identify
    sub: String,
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
    let claims = Claims {
        sub: user_id.to_owned(),
        exp: (Utc::now() + Duration::seconds(60)).timestamp() as usize,
    };

    let token = encode(
        &Header::new(Algorithm::HS256),
        &claims,
        &EncodingKey::from_secret(SECRET.as_bytes()),
    )?;
    Ok(token)
}

pub async fn get_jwt() -> String {
    generate_jwt(SECRET).unwrap()
}
