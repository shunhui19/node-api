use crate::{
    controllers::base::{ResultResponse, REQUEST_FAILED},
    middlewares::jwt::validate_jwt,
};
use axum::{http::HeaderMap, Json};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use tracing::{error, info, warn};

const BTC_MAINNET: &str = "https://bitcoin-rpc.publicnode.com";

pub async fn btc_handle(
    header: HeaderMap,
    Json(payload): Json<BTCRequest>,
) -> Json<ResultResponse> {
    let token = header.get("Authorization");
    if token.is_none() {
        error!("no token");
        return Json(ResultResponse {
            code: REQUEST_FAILED,
            msg: json!("no token"),
            data: Value::Null,
        });
    };

    // authorization jwt
    let auth_token = validate_jwt(token.unwrap().to_str().unwrap());
    match auth_token {
        Err(e) => {
            warn!("authorization no passed");
            return Json(ResultResponse {
                code: REQUEST_FAILED,
                msg: json!(e.to_string()),
                data: Value::Null,
            });
        }
        _ => info!("authorization passed"),
    };

    info!(
        "method: {:?}, params: {:?}, token: {:?}",
        &payload.method, &payload.params, token
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
