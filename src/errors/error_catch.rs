use rocket::{Request, Response};
use rocket::serde::json::Json;
use crate::comm::json_result::JsonResult;

#[catch(404)]
pub fn not_found(req: &Request)->Json<JsonResult<String>> {
    println!("404页面错误处理");
    Json(JsonResult::fail_for_code_mes(404,String::from("请求路径不存在！")))
}

// #[catch(401)]
// pub fn not_api_key(req: &Request) -> Json<JsonResult<String>>{
//     println!("401无授权");
//     println!("{:?}",req);
//     Json(JsonResult::fail_for_code_mes(401,String::from("")))
// }