use axum::{routing::get, Router, response::{IntoResponse, Json}, http::StatusCode};
use tokio::net::TcpListener;
use serde_json::json;

fn router() -> Router {
    Router::new().
        route("/hello", get(hello))
}

async fn listener(addr: &str, port: u16) -> TcpListener {
    let addr_port = format!("{}:{}", addr, port);
    TcpListener::bind(addr_port).await.unwrap()
}

async fn hello() -> impl IntoResponse {
    (StatusCode::OK, Json(json!({"message": "axum: Hello, world!"})))
}

#[tokio::main]
async fn main() {
    let app = router();

    let listener = listener("0.0.0.0", 8081);
    println!("launching server...");
    axum::serve(listener.await, app).await.unwrap();
}