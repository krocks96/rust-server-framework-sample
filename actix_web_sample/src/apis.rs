use actix_web::{get, App, HttpResponse, HttpServer, Responder};

// HTTPサーバ起動
pub async fn http_start(addr: &str, port: u16) -> std::io::Result<()> {
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
    HttpResponse::Ok().body("Hello world!")
}