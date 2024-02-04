use std::time::Duration;
use axum::{BoxError, middleware, Router};
use axum::error_handling::HandleErrorLayer;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::routing::get;
use tower::ServiceBuilder;

use crate::app_middleware::middlewares::app_logger;
use crate::controller::sys_controller;

static mut ROUTES: Vec<String> = Vec::new();


pub fn new_app() -> Router {
    Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/getUrl", get(sys_controller::get_url))
        .route("/panic", get(sys_controller::test_panic))
        .layer(middleware::from_fn(app_logger))

}

async fn err(x:BoxError) -> Response {
    StatusCode::INTERNAL_SERVER_ERROR
        .into_response()
}