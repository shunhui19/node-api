use crate::controllers::base::{ResultResponse, REQUEST_FAILED};
use axum::Json;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use tracing::{info, warn};

const BTC_MAINNET: &str = "https://bitcoin-rpc.publicnode.com";

pub async fn btc_handle(Json(payload): Json<BTCRequest>) -> Json<ResultResponse> {
    info!(
        "method: {:?}, params: {:?}",
        &payload.method, &payload.params
    );
    let resp = call(payload).await;
    match resp {
        Err(e) => {
            let mut err_msg = String::from("request failed, the reaseon is: ");
            err_msg.push_str(e.to_string().as_str());
            warn!(err_msg, "request failed, the reason");
            let result = ResultResponse {
                code: REQUEST_FAILED,
                msg: json!(err_msg),
                data: Value::Null,
            };
            Json(result)
        }
        Ok(response) => {
            info!(?response, "response from node: ");
            let result = ResultResponse {
                code: 200,
                msg: response.error,
                data: response.result,
            };
            Json(result)
        }
    }
}
async fn call(payload: BTCRequest) -> Result<BTCResponse, reqwest::Error> {
    let resp = Client::new()
        .post(BTC_MAINNET)
        .json(&payload)
        .send()
        .await?
        .json::<BTCResponse>()
        .await?;
    Ok(resp)
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BTCRequest {
    id: String,
    jsonrpc: String,
    method: String,
    params: Value,
}

#[derive(Deserialize, Debug)]
struct BTCResponse {
    result: Value,
    error: Value,
}
