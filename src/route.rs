use axum::Router;
use axum::routing::get;

use crate::errors::error_catch::not_found;

static mut ROUTES:Vec<String> = Vec::new();


pub fn new_app()->Router{
    Router::new()
        .route("/", get(|| async { "Hello, World!" }))
}