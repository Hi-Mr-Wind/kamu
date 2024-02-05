use axum::{
    http::Request,
    middleware::Next,
    response::Response,
};
use axum::body::Body;
use axum::http::{HeaderValue, StatusCode};
use crate::comm::json_result::JsonResult;

///自定义中间件记录日志
pub async fn app_logger(request: Request<Body>, next: Next, ) -> Response {
    // 对请求做一些处理
    info!("Request start:{} {} {:?}",request.method(),request.uri(),request.version());
    //调用下一个中间价
    let response = next.run(request).await;
    info!("response status:{})",response.status());

    // 对响应做一些处理，返回响应
    response
}

pub async fn err(request: Request<Body>, next: Next, ) -> Response{
    let mut x = next.run(request).await;
    if x.status().is_server_error() {
        let result = serde_json::to_string(&JsonResult::<String>::fail_for_code_mes(x.status().as_u16(), String::from("服务器内部错误"))).unwrap();
        let response = Response::builder()
            .status(x.status())
            .header("Content-Type","application/json;charset=UTF-8")
            .body(Body::from(result))
            .unwrap();
        return response
    }
    if !x.status().is_success() {
        let result = serde_json::to_string(&JsonResult::<String>::fail_for_code_mes(x.status().as_u16(), String::from(x.status().to_string()))).unwrap();
        let response = Response::builder()
            .status(x.status())
            .header("Content-Type","application/json;charset=UTF-8")
            .body(Body::from(result))
            .unwrap();
        return response
    };
    x
}

/// 校验token
pub async fn verify_key(request: Request<Body>, next: Next, ) -> Response<Body> {
    let option = request.headers().get("X-Auth-Token");
    match option {
        None => {
            info!("未找到token");
            let result = serde_json::to_string(&JsonResult::<String>::fail_for_code_mes(StatusCode::UNAUTHORIZED.as_u16(), String::from("非法用户"))).unwrap();
            let response = Response::builder()
                .status(StatusCode::UNAUTHORIZED)
                .header("Content-Type","application/json;charset=UTF-8")
                .body(Body::from(result))
                .unwrap();
            response
        }
        Some(tokio) => {
            info!("{:?}",tokio);
             next.run(request).await
        }
    }
}