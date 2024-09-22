use serde::Serialize;
use serde_json::Value;

pub const REQUEST_FAILED: i32 = -1;

#[derive(Serialize)]
pub struct ResultResponse {
    pub code: i32,
    pub msg: Value,
    pub data: Value,
}
