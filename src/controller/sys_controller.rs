use rocket::serde::json::Json;

use crate::comm::json_result::JsonResult;
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
pub fn test_post_json(user: Json<user::User>)->Json<JsonResult<user::User>>{
   println!("{:?}",user);
   Json(JsonResult::ok_for_data(Some(user.into_inner())))
}