use salvo::prelude::*;
use serde::Serialize;
// use serde_json::json;

#[derive(Serialize)]
struct Message {
    message: String,
}

#[tokio::main]
async fn main() {
    println!("launching server...");
    tracing_subscriber::fmt().init();

    let router = Router::new()
        .push(Router::new().path("hello").get(hello));
    let acceptor = TcpListener::new("0.0.0.0:8083").bind().await;
    Server::new(acceptor).serve(router).await;
}

#[handler]
async fn hello(res: &mut Response) {
    let data = Message {
        message: "Salvo: Hello, world!".to_string(),
    };
    res.render(Json(data));
}