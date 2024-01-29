use std::sync::atomic::{AtomicUsize, Ordering};

use rocket::{Data, Response};
use rocket::fairing::{Fairing, Info, Kind};
use rocket::form::validate::Len;
use rocket::http::{ContentType, Method, Status};
use rocket::request::{FromRequest, Outcome, Request};
use rocket::serde::Serialize;
use rocket::yansi::Paint;

#[derive(Serialize, Debug)]
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
            data: None,
        }
    }

    pub fn fail_for_code(code: i32) -> JsonResult<T> {
        JsonResult {
            code,
            mes: "失败".to_string(),
            data: None,
        }
    }
    pub fn fail_for_code_mes(code: i32, mes: String) -> JsonResult<T> {
        JsonResult {
            code,
            mes,
            data: None,
        }
    }
}

pub struct ApiKey<'r>(&'r str);

#[derive(Debug)]
pub enum ApiKeyError {
    Missing(JsonResult<String>),
    Invalid(JsonResult<String>),
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for ApiKey<'r> {
    type Error = ApiKeyError;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        //如果“key”是有效的 API 密钥字符串，则返回 true。
        fn is_valid(key: &str) -> bool {
            key == "valid_api_key"
        }

        match req.headers().get_one("x-api-key") {
            None => Outcome::Error((Status::Unauthorized, ApiKeyError::Missing(JsonResult::fail_for_code_mes(401, "找不到授权码".to_string())))),
            Some(key) => {
                if is_valid(key) {
                    Outcome::Success(ApiKey(key))
                } else {
                    Outcome::Error((Status::Unauthorized, ApiKeyError::Missing(JsonResult::fail_for_code_mes(401, "授权码过期".to_string()))))
                }
            }
        }
    }
}


pub struct Counter {
    get: AtomicUsize,
    post: AtomicUsize,
}
impl Counter {
    pub fn new() -> Counter {
        Counter {
            get: AtomicUsize::new(0),
            post: AtomicUsize::new(0),
        }
    }
}

#[rocket::async_trait]
impl Fairing  for Counter {
    // This is a request and response fairing named "GET/POST Counter".
    fn info(&self) -> Info {
        Info {
            name: "GET/POST Counter",
            kind: Kind::Request | Kind::Response
        }
    }

    async fn on_request(&self, req: &mut Request<'_>, _: &mut Data<'_>) {
        if req.method() == Method::Get {
            self.get.fetch_add(1, Ordering::Relaxed);
        } else if req.method() == Method::Post {
            self.post.fetch_add(1, Ordering::Relaxed);
        }
    }

    async fn on_response<'r>(&self, req: &'r Request<'_>, res: &mut Response<'r>) {
        if res.status()==Status::Unauthorized{
            println!("后置请求");
            let result: JsonResult<String> = JsonResult::fail_for_code_mes(401, String::from("未找到授权"));
            let body = serde_json::to_string(&result).unwrap();
            res.set_status(Status::Ok);
            res.set_header(ContentType::JSON);
            res.set_sized_body(body.len(),std::io::Cursor::new(body));
        }
    }
}