use poem::{get, handler, listener::TcpListener, web::Path, web::Json, IntoResponse, Route, Server};
use serde::Serialize;

#[derive(Serialize)]
struct Message {
    message: String,
}

#[handler]
fn hello() -> Json<Message> {
    Json(Message {
        message: "poem: Hello, world!".to_string(),
    })
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let app = Route::new().at("/hello", get(hello));
    Server::new(TcpListener::bind("0.0.0.0:8084"))
        .run(app)
        .await
}