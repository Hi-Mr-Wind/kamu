use axum::http::StatusCode;
use axum::Json;

use crate::comm::json_result::JsonResult;
use crate::entity::OsInfo;
use crate::entity::user::User;
use crate::errors::kamu_error::{AppError, KaMuError};
use crate::service::core_service::CoreService;

pub async fn get_url() -> Json<JsonResult<String>> {
    CoreService::login(User::default());
    Json(JsonResult::ok(String::from("成功")))
}

pub async fn get_os_info()->Json<JsonResult<OsInfo>>{
    let os_info = CoreService::get_os_info();
    Json(JsonResult::ok_for_data(Some(os_info)))
}

pub async fn test_panic() -> Result<Json<JsonResult<String>>,AppError> {
    // Json (JsonResult::fail_for_code(StatusCode::INTERNAL_SERVER_ERROR.as_u16()))
   let err =  AppError::new(505,"测试的错误".to_string(),KaMuError::RunTimeError);
    Err(err)
}