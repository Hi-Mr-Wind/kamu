use axum::{extract, Json};

use crate::comm::json_result::{JR, JsonResult};
use crate::errors::kamu_error::{AppError, KaMuError};

pub async fn get_url() -> extract::Json<JR<String>> {
    Json(JsonResult::ok(String::from("成功")))
}

pub async fn test_panic() -> AppError {
    AppError::new(500,String::from("失败"),KaMuError::RunTimeError)
}