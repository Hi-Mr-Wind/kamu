use std::any::Any;
use rocket::serde::Serialize;

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct JsonResult<T> {
    pub code: i32,
    pub mes: String,
    pub data: Option<T>,
}

impl<T> JsonResult<T> {
    pub fn new(code: i32, mes: String, data: Option<T>) -> JsonResult<T> {
        JsonResult {
            code,
            mes,
            data,
        }
    }

    pub fn ok(mes: String) -> JsonResult<T> {
        JsonResult {
            code: 200,
            mes,
            data: None,
        }
    }

    pub fn ok_for_data(data: Option<T>) -> JsonResult<T> {
        JsonResult {
            code: 200,
            mes: "成功".to_string(),
            data,
        }
    }

    pub fn fail() -> JsonResult<T> {
        JsonResult {
            code: 400,
            mes: "失败".to_string(),
            data:None,
        }
    }

    pub fn fail_for_code(code: i32) ->JsonResult<T> {
        JsonResult{
            code,
            mes:"失败".to_string(),
            data: None,
        }
    }
}