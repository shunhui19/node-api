use std::{i16, i32};

use sqlx::{types::time::OffsetDateTime, Error};

use crate::configs::db;

struct Tokens {
    id: i32,
    user_id: i32,
    token: String,
    created_at: Option<OffsetDateTime>,
    expires_at: Option<OffsetDateTime>,
    revoked: i16,
    current_request_count: i32,
}

impl Tokens {
    pub async fn get_token_by_user_id(user_id: i32) -> Result<Self, Error> {
        let pool = db::get_pg_pool().await;
        let token = sqlx::query_as!(Tokens, "SELECT * FROM tokens WHERE user_id = $1", user_id)
            .fetch_one(pool)
            .await?;
        Ok(token)
    }

    pub async fn create_token(
        user_id: i32,
        token: String,
        expires: OffsetDateTime,
    ) -> Result<i32, Error> {
        let pool = db::get_pg_pool().await;
        let row = sqlx::query!(
            "INSERT INTO tokens (user_id, token, expires_at) VALUES ($1, $2, $3) RETURNING id",
            user_id,
            token,
            expires
        )
        .fetch_one(pool)
        .await?;

        Ok(row.id)
    }

    pub async fn update_expires_by_user_id(
        user_id: i32,
        expires: OffsetDateTime,
    ) -> Result<(), Error> {
        let pool = db::get_pg_pool().await;
        let _ = sqlx::query!(
            "UPDATE TOKENS SET expires_at = $1 WHERE user_id = $2",
            expires,
            user_id
        )
        .fetch_one(pool)
        .await?;
        Ok(())
    }

    pub async fn update_revoked_by_user_id(user_id: i32, revoked: i16) -> Result<(), Error> {
        let pool = db::get_pg_pool().await;
        let _ = sqlx::query!(
            "UPDATE tokens SET revoked = $1 WHERE user_id = $2",
            revoked,
            user_id
        )
        .fetch_one(pool)
        .await?;
        Ok(())
    }
}
