use sqlx::types::time::OffsetDateTime;

#[derive(sqlx::FromRow, Debug)]
struct TokensRequests {
    id: i32,
    token_id: i32,
    request_time: Option<OffsetDateTime>,
    chain: String,
    method: String,
    response_time: i16,
    response_status: i16,
    created_at: Option<OffsetDateTime>,
}
