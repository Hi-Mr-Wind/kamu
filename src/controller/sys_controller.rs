use axum::http::StatusCode;
use axum::Json;

use crate::comm::json_result::JsonResult;
use crate::entity::OsInfo;
use crate::entity::user::User;
use crate::service::core_service::CoreService;

pub async fn get_url() -> Json<JsonResult<String>> {
    CoreService::login(User::default());
    Json(JsonResult::ok(String::from("成功")))
}

pub async fn get_os_info()->Json<JsonResult<OsInfo>>{
    let os_info = CoreService::get_os_info();
    Json(JsonResult::ok_for_data(Some(os_info)))
}

pub async fn test_panic() -> Json<JsonResult<String>> {
    Json (JsonResult::fail_for_code(StatusCode::INTERNAL_SERVER_ERROR.as_u16()))
}