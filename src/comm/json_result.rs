use std::io::Error;
use std::sync::atomic::AtomicUsize;

use serde::{Deserialize, Serialize};
use crate::errors::kamu_error::{AppError, KaMuError};

pub type JR<T> = Result<JsonResult<T>,AppError>;

#[derive(Debug,Deserialize,Serialize)]
pub struct JsonResult<T> {
    pub code: i32,
    pub mes: String,
    pub data: Option<T>,
}

impl<T> JsonResult<T> {
    pub fn new(code: i32, mes: String, data: Option<T>) -> JR<T> {
        Ok(JsonResult {
            code,
            mes,
            data,
        })
    }
    pub fn ok(mes: String) -> JR<T> {
        Ok(JsonResult {
            code: 200,
            mes,
            data: None,
        })
    }
    pub fn ok_for_data(data: Option<T>) -> JR<T> {
        Ok(JsonResult {
            code: 200,
            mes: "成功".to_string(),
            data,
        })
    }
    pub fn fail() -> JR<T> {
        Err(AppError::new(400, "失败".to_string(), KaMuError::RunTimeError))
    }
    pub fn fail_for_code(code: i32) -> JR<T> {
        Err(AppError::new(code,String::from("失败"),KaMuError::RunTimeError))
    }
    pub fn fail_for_code_mes(code: i32, mes: String) -> JR<T> {
        Err(AppError::new(code,mes,KaMuError::RunTimeError))
    }
}
