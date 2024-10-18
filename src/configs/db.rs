use core::panic;

use serde::Deserialize;
use sqlx::{postgres::PgPoolOptions, PgPool};

use crate::{CONFIG, DB_POOL};

#[derive(Deserialize, Debug)]
pub struct Db {
    #[allow(dead_code)]
    host: String,
    #[allow(dead_code)]
    port: u64,
    #[allow(dead_code)]
    username: String,
    #[allow(dead_code)]
    password: String,
    #[allow(dead_code)]
    db_name: String,
}

/// initialize the pg connect pools.
async fn initialize_pg_pool() -> PgPool {
    let connect_url = format!(
        "postgres://{}:{}@{}:{}/{}",
        CONFIG.db.username, CONFIG.db.password, CONFIG.db.host, CONFIG.db.port, CONFIG.db.db_name
    );
    match PgPoolOptions::new()
        .max_connections(5)
        .connect(connect_url.as_str())
        .await
    {
        Ok(p) => p,
        Err(_) => {
            panic!("Faile to initialize postgres pool")
        }
    }
}

/// get a pg connect from PgPool.
pub async fn get_pg_pool() -> &'static PgPool {
    DB_POOL
        .get_or_init(|| async { initialize_pg_pool().await })
        .await
}

#[cfg(test)]
mod test {
    use super::initialize_pg_pool;
    use tokio::test;

    #[test]
    async fn initialize_pg_pool_test() {
        let pool = initialize_pg_pool().await;

        let res =
            sqlx::query("INSERT INTO USERS (username, email, password_hash) VALUES ($1, $2, $3)")
                .bind("xuan.kuang")
                .bind("xuan@163.com")
                .bind("123456")
                .execute(&pool)
                .await;
        match res {
            Ok(num) => println!("success insert {:?} row into database", num),
            Err(e) => println!("failed to insert data into database, reason:{:?}", e),
        }
    }
}
