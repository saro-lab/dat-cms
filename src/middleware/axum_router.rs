use axum::body::Body;
use axum::extract::ConnectInfo;
use axum::http::{Request, StatusCode};
use axum::middleware::Next;
use axum::response::{IntoResponse, Response};
use std::any::Any;
use std::net::{IpAddr, SocketAddr};

const TEXT_PLAIN: &[(&str, &str); 2] = &[("Content-Type", "text/plain"), ("Err", "")];

pub fn handle_panic(err: Box<dyn Any + Send>) -> Response {
    let details = if let Some(s) = err.downcast_ref::<&str>() {
        s.to_string()
    } else if let Some(s) = err.downcast_ref::<String>() {
        s.clone()
    } else {
        format!("{:?}", err)
    };
    tracing::error!("{details}");
    (StatusCode::INTERNAL_SERVER_ERROR, *TEXT_PLAIN, "").into_response()
}

pub async fn handle_error(req: Request<Body>, next: Next) -> Response {
    let uri = req.uri().to_string();
    let response = next.run(req).await;
    let status = response.status();

    if
        status.is_success() ||
        // *APPLICATION_JSON 에 같이 있는 Err을 통해 중복 실행 방어
        response.headers().contains_key("Err")
    {
        response
    } else {
        if status == StatusCode::NOT_FOUND {
            tracing::info!("404: {}", uri);
            (StatusCode::BAD_REQUEST, *TEXT_PLAIN).into_response()
        } else if status.is_client_error() {
            (StatusCode::BAD_REQUEST, *TEXT_PLAIN).into_response()
        } else {
            tracing::info!("{}: {}", status, uri);
            (StatusCode::INTERNAL_SERVER_ERROR, *TEXT_PLAIN).into_response()
        }
    }
}

pub async fn session_layout(ConnectInfo(socket_addr): ConnectInfo<SocketAddr>, mut req: Request<Body>, next: Next, ) -> Result<Response, StatusCode> {
    let client_ip = req
        .headers_mut()
        .get("X-Forwarded-For")
        .and_then(|x| x.to_str().ok())
        .and_then(|x| x.split(',').next())
        .and_then(|x| x.trim().parse::<IpAddr>().ok())
        .unwrap_or_else(|| socket_addr.ip());
    req.extensions_mut().insert(client_ip);
    Ok(next.run(req).await)
}
