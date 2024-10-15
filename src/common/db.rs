#[cfg(test)]
mod test {
    use sqlx::postgres::PgPoolOptions;

    #[tokio::test]
    async fn connect_pg_test() -> Result<(), sqlx::Error> {
        // create pg connect.
        let database_url = "postgres://postgresql:postgresql@localhost/node-api";
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(database_url)
            .await?;

        let _rows_affected =
            sqlx::query("INSERT INTO USERS (username, email, password_hash) VALUES ($1, $2, $3)")
                .bind("frank.kuang")
                .bind("shunhui29@163.com")
                .bind("123456")
                .execute(&pool)
                .await?;

        // query data.



        Ok(())
    }
}
