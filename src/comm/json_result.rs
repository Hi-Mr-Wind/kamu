use std::io::Error;
use std::sync::atomic::AtomicUsize;
use axum::http::{header, StatusCode};
use axum::response::{IntoResponse, Response};

use serde::{Deserialize, Serialize};
use crate::errors::kamu_error::{AppError, KaMuError};


#[derive(Debug, Deserialize, Serialize)]
pub struct JsonResult<T> {
    pub code: u16,
    pub mes: String,
    pub data: Option<T>,
}

impl<T> JsonResult<T> {
    pub fn new(code: u16, mes: String, data: Option<T>) -> JsonResult<T> {
        JsonResult {
            code,
            mes,
            data,
        }
    }
    pub fn ok(mes: String) -> JsonResult<T> {
        JsonResult {
            code: StatusCode::OK.as_u16(),
            mes,
            data: None,
        }
    }
    pub fn ok_for_data(data: Option<T>) -> JsonResult<T> {
        JsonResult {
            code: StatusCode::OK.as_u16(),
            mes: "成功".to_string(),
            data,
        }
    }
    pub fn fail() -> JsonResult<T> {
        JsonResult{
            code:400,
            mes:String::from("失败"),
            data:None
        }
    }
    pub fn fail_for_code(code: u16) -> JsonResult<T> {
        JsonResult{
            code,
            mes:String::from("失败"),
            data:None
        }
    }
    pub fn fail_for_code_mes(code: u16, mes: String) -> JsonResult<T> {
        JsonResult{
            code,
            mes,
            data:None
        }
    }
}