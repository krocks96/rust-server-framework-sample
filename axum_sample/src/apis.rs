use axum::{routing::get, Router, response::{IntoResponse, Json}, http::StatusCode};
use tokio::net::TcpListener;
use serde_json::json;

pub fn router() -> Router {
    Router::new().
        route("/hello", get(hello))
}

pub async fn listener(addr: &str, port: u16) -> TcpListener {
    let addr_port = format!("{}:{}", addr, port);
    TcpListener::bind(addr_port).await.unwrap()
}

async fn hello() -> impl IntoResponse {
    (StatusCode::OK, Json(json!({"message": "Hello, world!"})))
}