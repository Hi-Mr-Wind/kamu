use std::fmt::Debug;

use axum::{middleware, Router};
use axum::routing::get;

use crate::app_middleware::middlewares as mid;
use crate::controller::sys_controller;

static mut ROUTES: Vec<String> = Vec::new();


pub fn new_app() -> Router {
    info!("Service loading.....");
    let router = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/getUrl", get(sys_controller::get_url))
        .route("/panic", get(sys_controller::test_panic))
        .layer(middleware::from_fn(mid::verify_key));
    let r = Router::new()
        .nest("/api", router)
        .layer(middleware::from_fn(mid::app_logger))
        .layer(middleware::from_fn(mid::err))
        .fallback(mid::fallback);
    info!("Service started successfully!");
    r
}