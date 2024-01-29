use rocket::{Request, Rocket};
use rocket::http::hyper::body::HttpBody;
use rocket::serde::json::Json;
use rocket::yansi::Paint;

use crate::comm::json_result::{ApiKey, JsonResult};
use crate::entity::user;

#[get("/")]
pub fn index() -> Json<JsonResult<String>> {
   Json(JsonResult::ok("成功".to_string()))
}

#[get("/fail")]
pub fn fail()->Json<JsonResult<String>>{
   Json(JsonResult::fail_for_code(400))
}

#[post("/pptest",data = "<user>")]
pub async fn test_post_json(user: Json<user::User>)->Json<JsonResult<user::User>>{
   println!("{:?}",user);
   Json(JsonResult::ok_for_data(Some(user.into_inner())))
}

#[get("/apikey")]
pub async fn test_key(apiKey: ApiKey<'_>)->Json<JsonResult<String>>{
   Json(JsonResult::ok("成功".to_string()))
}