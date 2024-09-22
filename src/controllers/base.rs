use serde::Serialize;
use serde_json::Value;

#[derive(Serialize)]
pub struct ResultResponse {
    pub code: i32,
    pub msg: Value,
    pub data: Value,
}
