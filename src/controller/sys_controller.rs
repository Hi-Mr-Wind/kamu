use axum::{extract, Json};
use axum::http::StatusCode;

use crate::comm::json_result::{JsonResult};

pub async fn get_url() -> extract::Json<JsonResult<String>> {
    Json(JsonResult::ok(String::from("成功")))
}

pub async fn test_panic() -> Json<JsonResult<String>> {
    Json (JsonResult::fail_for_code(StatusCode::INTERNAL_SERVER_ERROR.as_u16()))
}