use std::fmt::Debug;

use axum::http::{StatusCode, Uri};
use axum::{middleware, Router};
use axum::routing::get;

use crate::app_middleware::middlewares as mid;
use crate::controller::sys_controller;

static mut ROUTES: Vec<String> = Vec::new();


pub fn new_app() -> Router {
    let router = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/getUrl", get(sys_controller::get_url))
        .route("/panic", get(sys_controller::test_panic))
        .layer(middleware::from_fn(mid::verify_key))
        .fallback(fallback);
    Router::new()
        .nest("/api",router)
        .layer(middleware::from_fn(mid::app_logger))
        .layer(middleware::from_fn(mid::err))
}
/// 不存在的路由
async fn fallback(uri: Uri) -> (StatusCode, String) {
    info!("No route for {}",&uri);
    (StatusCode::NOT_FOUND, format!("No route for {uri}"))
}