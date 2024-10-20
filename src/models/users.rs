use crate::configs::db;
use sqlx::{types::time::OffsetDateTime, Error};

#[derive(Debug, sqlx::FromRow)]
struct Users {
    #[allow(dead_code)]
    id: i32,
    username: String,
    email: String,
    password_hash: String,
    is_active: i16,
    #[allow(dead_code)]
    created_at: Option<OffsetDateTime>,
    #[allow(dead_code)]
    updated_at: Option<OffsetDateTime>,
}

impl Users {
    pub async fn get_user_by_username(name: &str) -> Result<Self, Error> {
        let pool = db::get_pg_pool().await;
        // let user = sqlx::query_as("SELECT * FROM users WHERE username = $1")
        //     .bind(name)
        //     .fetch_one(pool)
        //     .await?;
        let user = sqlx::query_as!(Users, "SELECT * FROM users WHERE username = $1", name)
            .fetch_one(pool)
            .await?;
        Ok(user)
    }

    pub async fn register_user(
        username: String,
        password: String,
        email: String,
    ) -> Result<i32, Error> {
        let pool = db::get_pg_pool().await;
        let row = sqlx::query!(
            "INSERT INTO users (username, email, password_hash) VALUES ($1, $2, $3) RETURNING id",
            username,
            email,
            password
        )
        .fetch_one(pool)
        .await?;

        Ok(row.id)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use sqlx::test;

    #[test]
    fn find_by_username_test() {
        // match Users::find_by_username("frank.kuang").await {
        //     Ok(u) => {
        //         assert_eq!(u.email, "shunhui29@163.com");
        //     }
        //     Err(e) => {
        //         println!("error:{:?}", e);
        //     }
        // };
        assert_eq!(
            Users::get_user_by_username("frank.kuang")
                .await
                .unwrap()
                .email,
            "shunhui29@163.com"
        );
    }
}
