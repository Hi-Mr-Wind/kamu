use std::error::Error;
use std::fmt::{Display, Formatter};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::routing::head;
use serde::{Deserialize, Serialize};
use serde_json::json;
use crate::errors::kamu_error::KaMuError::RunTimeError;


///程序内部异常
#[derive(Debug,Deserialize,Serialize)]
pub enum  KaMuError{
    ///自定义运行时异常
    RunTimeError,
    ///文件异常
    FileError,
    ///转存异常
    UnloadingError
}

#[derive(Debug,Deserialize,Serialize)]
pub struct AppError{
    pub code:i32,
    pub mes:String,
    pub data: KaMuError
}

impl AppError {
    pub fn new(code:i32, mes:String, data:KaMuError) ->AppError{
        AppError{
            code,
            mes,
            data
        }
    }
}
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        (
            // StatusCode::INTERNAL_SERVER_ERROR,
            StatusCode::INTERNAL_SERVER_ERROR,
            serde_json::to_string(&self).unwrap(),
        )
            .into_response()
    }
}

impl Display for AppError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f,"Error:{}",self.to_string())
    }
}

impl Error for AppError {}