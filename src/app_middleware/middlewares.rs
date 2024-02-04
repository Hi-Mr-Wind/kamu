use axum::{
    http::Request,
    middleware::Next,
    response::Response,
};
use axum::body::Body;

///自定义中间件方法
pub async fn app_logger(request: Request<Body>, next: Next, ) -> Response {
    // 对请求做一些处理
    debug!("Request start:{} {} {:?}",request.method(),request.uri(),request.version());
    //......

    //调用下一个中间价
    let response = next.run(request).await;

    // debug!("{:?}",response);

    // 对响应做一些处理，返回响应
    response
}