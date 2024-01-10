use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use serde_json::json;

// HTTPサーバ起動
async fn http_start(addr: &str, port: u16) -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
    })
    .bind((addr, port))?
    .run()
    .await
}

#[get("/hello")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().json(json!({"message": "actix: Hello, world!"}))
}

#[actix_web::main]
pub async fn main() -> std::io::Result<()> {
    println!("launching server...");
    http_start("0.0.0.0", 8080).await
}
